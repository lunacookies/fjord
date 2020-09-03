//! Implementation of the Fjord interpreter and related types.

mod error;
pub use error::EvalError;
use error::EvalErrorKind;

use crate::ast::{
    BinOp, BindingDef, BindingUsage, Digits, Expr, ExprKind, FunctionCall, Item, ItemKind, Lambda,
    Root, StringLiteral,
};
use crate::env::Env;
use crate::val::Val;
use crate::Op;
use std::cmp::Ordering;
use text_size::TextRange;

impl Root {
    pub(crate) fn eval(&self, env: &mut Env<'_>) -> Result<Val, EvalError> {
        let items: Vec<_> = self.items().collect();

        let num_items = items.len();
        let at_last = |idx| idx == num_items - 1;

        if items.is_empty() {
            return Ok(Val::Nil);
        }

        for (idx, item) in items.iter().enumerate() {
            let eval_output = item.eval(env)?;
            if at_last(idx) {
                return Ok(eval_output);
            }
        }

        // All lists are either empty (see the is_empty call before the for loop) or have a last
        // item (see at_last call above), so we are guaranteed to have returned by this point.
        unreachable!()
    }
}

impl Item {
    fn eval(&self, env: &mut Env<'_>) -> Result<Val, EvalError> {
        match self.kind() {
            ItemKind::BindingDef(binding_def) => {
                binding_def.eval(env)?;
                Ok(Val::Nil)
            }
            ItemKind::Expr(expr) => expr.eval(env),
        }
    }
}

impl BindingDef {
    fn eval(&self, env: &mut Env<'_>) -> Result<(), EvalError> {
        let expr = self.expr().unwrap().eval(env)?;
        let name = self.binding_name().unwrap();

        env.store_binding(name, expr);

        Ok(())
    }
}

impl Expr {
    fn eval(&self, env: &Env<'_>) -> Result<Val, EvalError> {
        match self.kind() {
            ExprKind::BinOp(bin_op) => bin_op.eval(env),
            ExprKind::FunctionCall(function_call) => function_call.eval(env),
            ExprKind::Lambda(lambda) => Ok(Val::Lambda(lambda)),
            ExprKind::BindingUsage(binding_usage) => binding_usage.eval(env),
            ExprKind::StringLiteral(string_literal) => Ok(string_literal.eval()),
            ExprKind::NumberLiteral(digits) => Ok(digits.eval()),
        }
    }
}

impl BinOp {
    fn eval(&self, env: &Env<'_>) -> Result<Val, EvalError> {
        let op = self.op().unwrap().as_op().unwrap();

        let lhs = self.lhs().unwrap().eval(env)?;
        let rhs = self.rhs().unwrap().eval(env)?;

        match (lhs, rhs) {
            (Val::Number(lhs), Val::Number(rhs)) => {
                let result = match op {
                    Op::Add => lhs + rhs,
                    Op::Sub => lhs - rhs,
                    Op::Mul => lhs * rhs,
                    Op::Div => lhs / rhs,
                };

                Ok(Val::Number(result))
            }
            (lhs, rhs) => {
                let error_kind = EvalErrorKind::BinOpOnNonNumbers {
                    lhs_ty: lhs.ty(),
                    rhs_ty: rhs.ty(),
                };

                Err(EvalError::new(error_kind, self.text_range()))
            }
        }
    }
}

impl FunctionCall {
    fn eval(&self, env: &Env<'_>) -> Result<Val, EvalError> {
        let name = self.name().unwrap();

        let val = env
            .get_binding(name.text())
            .ok_or_else(|| EvalError::new(EvalErrorKind::BindingDoesNotExist, name.text_range()))?;

        if let Val::Lambda(lambda) = val {
            let params: Result<Vec<_>, _> = self
                .param_exprs()
                .unwrap()
                .map(|param| param.eval(env))
                .collect();

            let params = params?;
            let params_range = self.params().unwrap().text_range();

            lambda.eval(params_range, params.into_iter(), env)
        } else {
            Err(EvalError::new(
                EvalErrorKind::CallNonLambda,
                name.text_range(),
            ))
        }
    }
}

impl Lambda {
    fn eval(
        &self,
        call_params_range: TextRange,
        params: impl ExactSizeIterator<Item = Val>,
        env: &Env<'_>,
    ) -> Result<Val, EvalError> {
        let mut new_env = env.create_child();

        match params.len().cmp(&self.param_names().unwrap().count()) {
            Ordering::Less => {
                return Err(EvalError::new(
                    EvalErrorKind::TooFewParams,
                    call_params_range,
                ));
            }
            Ordering::Greater => {
                return Err(EvalError::new(
                    EvalErrorKind::TooManyParams,
                    call_params_range,
                ));
            }
            Ordering::Equal => {}
        }

        for (param_name, param_val) in self.param_names().unwrap().zip(params) {
            new_env.store_binding(param_name, param_val);
        }

        self.body().unwrap().eval(&new_env)
    }
}

impl BindingUsage {
    fn eval(&self, env: &Env<'_>) -> Result<Val, EvalError> {
        let binding_name = self.binding_name().unwrap();

        env.get_binding(&binding_name)
            .ok_or_else(|| EvalError::new(EvalErrorKind::BindingDoesNotExist, self.text_range()))
    }
}

impl StringLiteral {
    fn eval(&self) -> Val {
        let text = self.text();

        // Slice off quotes.
        Val::Str(text[1..text.len() - 1].to_string())
    }
}

impl Digits {
    fn eval(&self) -> Val {
        Val::Number(self.text().parse().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::expr::{parse_binding_usage, parse_expr, parse_lambda};
    use crate::parser::item::parse_binding_def;
    use crate::parser::Parser;

    #[test]
    fn evaluate_non_existent_binding_usage() {
        let mut p = Parser::new("$test");
        parse_binding_usage(&mut p);

        let syntax_node = p.finish_and_get_syntax();
        let binding_usage = BindingUsage::cast(syntax_node).unwrap();

        let env = Env::new(Vec::new()).unwrap();

        assert_eq!(
            binding_usage.eval(&env),
            Err(EvalError::new(
                EvalErrorKind::BindingDoesNotExist,
                TextRange::new(0.into(), 5.into()),
            ))
        );
    }

    #[test]
    fn evaluate_binding_usage_that_does_exist() {
        let mut p = Parser::new("$foo-bar");
        parse_binding_usage(&mut p);

        let syntax_node = p.finish_and_get_syntax();
        let binding_usage = BindingUsage::cast(syntax_node).unwrap();

        let mut env = Env::new(Vec::new()).unwrap();
        env.store_binding("foo-bar".into(), Val::Number(5));

        assert_eq!(binding_usage.eval(&env), Ok(Val::Number(5)));
    }

    #[test]
    fn evaluate_lambda() {
        let id_lambda = {
            let mut p = Parser::new("|x| $x");
            parse_lambda(&mut p);

            let syntax_node = p.finish_and_get_syntax();

            Lambda::cast(syntax_node).unwrap()
        };

        let apply_a_to_b_lambda = {
            let mut p = Parser::new("|a b| a $b");
            parse_lambda(&mut p);

            let syntax_node = p.finish_and_get_syntax();

            Lambda::cast(syntax_node).unwrap()
        };

        let env = Env::new(Vec::new()).unwrap();

        // Applying id lambda to "hello" gives "hello".
        assert_eq!(
            apply_a_to_b_lambda.eval(
                TextRange::default(),
                vec![Val::Lambda(id_lambda), Val::Str("hello".to_string())].into_iter(),
                &env,
            ),
            Ok(Val::Str("hello".to_string())),
        );
    }

    #[test]
    fn evaluate_lambda_with_too_many_params() {
        let id_lambda = {
            let mut p = Parser::new("|a| $a");
            parse_lambda(&mut p);

            let syntax_node = p.finish_and_get_syntax();

            Lambda::cast(syntax_node).unwrap()
        };

        let env = Env::new(Vec::new()).unwrap();

        // Dummy value.
        let call_range = TextRange::new(0.into(), 10.into());

        assert_eq!(
            id_lambda.eval(
                call_range,
                vec![Val::Number(5), Val::Str("test".to_string())].into_iter(),
                &env,
            ),
            Err(EvalError::new(EvalErrorKind::TooManyParams, call_range)),
        );
    }

    #[test]
    fn evaluate_lambda_with_too_few_params() {
        let ls_two_dirs_lambda = {
            let mut p = Parser::new("|dir1 dir2| ls $dir1 $dir2");
            parse_lambda(&mut p);

            let syntax_node = p.finish_and_get_syntax();

            Lambda::cast(syntax_node).unwrap()
        };

        let env = Env::new(Vec::new()).unwrap();

        // Dummy value.
        let call_range = TextRange::new(0.into(), 10.into());

        assert_eq!(
            ls_two_dirs_lambda.eval(
                call_range,
                vec![Val::Str("~/Documents".to_string())].into_iter(),
                &env,
            ),
            Err(EvalError::new(EvalErrorKind::TooFewParams, call_range)),
        );
    }

    #[test]
    fn call_lambda_with_several_params() {
        let mut env = Env::new(Vec::new()).unwrap();

        let return_first_lambda = {
            let mut p = Parser::new("|a b| $a");
            parse_lambda(&mut p);

            let syntax_node = p.finish_and_get_syntax();

            Lambda::cast(syntax_node).unwrap()
        };

        env.store_binding("return-first".into(), Val::Lambda(return_first_lambda));

        let return_first_application = {
            let mut p = Parser::new("return-first 5 10");
            parse_expr(&mut p);

            let syntax_node = p.finish_and_get_syntax();

            FunctionCall::cast(syntax_node).unwrap()
        };

        assert_eq!(return_first_application.eval(&env), Ok(Val::Number(5)));
    }

    #[test]
    fn call_lambda_without_any_params() {
        let mut env = Env::new(Vec::new()).unwrap();

        let always_return_100_lambda = {
            let mut p = Parser::new("|| 100");
            parse_lambda(&mut p);

            let syntax_node = p.finish_and_get_syntax();

            Lambda::cast(syntax_node).unwrap()
        };

        env.store_binding(
            "always-return-100".into(),
            Val::Lambda(always_return_100_lambda),
        );

        let always_return_100_application = {
            let mut p = Parser::new("always-return-100");
            parse_expr(&mut p);

            let syntax_node = p.finish_and_get_syntax();

            FunctionCall::cast(syntax_node).unwrap()
        };

        assert_eq!(
            always_return_100_application.eval(&env),
            Ok(Val::Number(100)),
        );
    }

    #[test]
    fn call_non_lambda() {
        let mut env = Env::new(Vec::new()).unwrap();
        env.store_binding("foo".into(), Val::Number(100));

        let call = {
            let mut p = Parser::new("foo 10");
            parse_expr(&mut p);

            let syntax_node = p.finish_and_get_syntax();

            FunctionCall::cast(syntax_node).unwrap()
        };

        assert_eq!(
            call.eval(&env),
            Err(EvalError::new(
                EvalErrorKind::CallNonLambda,
                TextRange::new(0.into(), 3.into()),
            )),
        );
    }

    #[test]
    fn evaluate_binding_def() {
        let binding_def = {
            let mut p = Parser::new("let a = 5");
            parse_binding_def(&mut p);

            let syntax_node = p.finish_and_get_syntax();

            BindingDef::cast(syntax_node).unwrap()
        };

        assert_eq!(
            {
                let mut env = Env::new(Vec::new()).unwrap();
                binding_def.eval(&mut env).unwrap();
                env
            },
            {
                let mut env = Env::new(Vec::new()).unwrap();
                env.store_binding("a".into(), Val::Number(5));
                env
            },
        );
    }

    #[test]
    fn evaluate_empty_root() {
        let root = {
            let p = Parser::new("");
            let syntax_node = p.parse().syntax();

            Root::cast(syntax_node).unwrap()
        };

        let mut env = Env::new(Vec::new()).unwrap();

        assert_eq!(root.eval(&mut env), Ok(Val::Nil));
    }

    #[test]
    fn evaluate_root_with_one_expr_returns_value_of_expr() {
        let root = {
            let p = Parser::new(r#""hello""#);
            let syntax_node = p.parse().syntax();

            Root::cast(syntax_node).unwrap()
        };

        let mut env = Env::new(Vec::new()).unwrap();

        assert_eq!(root.eval(&mut env), Ok(Val::Str("hello".to_string())));
    }

    #[test]
    fn evaluate_root_with_one_statement_returns_nil() {
        let root = {
            let p = Parser::new("let x = 1");
            let syntax_node = p.parse().syntax();

            Root::cast(syntax_node).unwrap()
        };

        let mut env = Env::new(Vec::new()).unwrap();

        assert_eq!(root.eval(&mut env), Ok(Val::Nil));
    }

    #[test]
    fn evaluate_root_with_multiple_expressions_returns_last() {
        let root = {
            let p = Parser::new(
                r#"
5
10
"foobar"
2"#,
            );
            let syntax_node = p.parse().syntax();

            Root::cast(syntax_node).unwrap()
        };

        let mut env = Env::new(Vec::new()).unwrap();

        assert_eq!(root.eval(&mut env), Ok(Val::Number(2)));
    }
}
