//! Implementation of the Fjord interpreter and related types.

mod error;
pub use error::EvalError;

use crate::ast::{
    BindingDef, BindingUsage, Digits, Expr, ExprKind, FunctionCall, Item, ItemKind, Lambda, Root,
    Statement, StatementKind, StringLiteral,
};
use crate::env::Env;
use crate::val::Val;

impl Root {
    pub(crate) fn eval(&self, env: &mut Env<'_>) -> Result<Val, EvalError> {
        let items: Vec<_> = self.items().collect();

        if items.is_empty() {
            return Ok(Val::Nil);
        }

        // We process the last item seperately to allow for implicit return.

        for item in &items[1..] {
            // If we’re at a return statement, we early return with the value of the return
            // statement.
            if let ItemKind::Statement(statement) = item.kind() {
                if let StatementKind::ReturnStatement(return_statement) = statement.kind() {
                    // If the return statement does not have a value, we return with Nil.
                    return return_statement
                        .val()
                        .map(|expr| expr.eval(env))
                        .unwrap_or(Ok(Val::Nil));
                }
            }

            item.eval(env)?;
        }

        let last_item = items.last().unwrap();
        last_item.eval(env)
    }
}

impl Item {
    fn eval(&self, env: &mut Env<'_>) -> Result<Val, EvalError> {
        match self.kind() {
            ItemKind::Statement(statement) => {
                statement.eval(env)?;
                Ok(Val::Nil)
            }
            ItemKind::Expr(expr) => expr.eval(env),
        }
    }
}

impl Statement {
    fn eval(&self, env: &mut Env<'_>) -> Result<(), EvalError> {
        match self.kind() {
            StatementKind::BindingDef(binding_def) => binding_def.eval(env),
            StatementKind::ReturnStatement(_) => Ok(()),
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
            ExprKind::FunctionCall(function_call) => function_call.eval(env),
            ExprKind::Lambda(lambda) => Ok(Val::Lambda(lambda)),
            ExprKind::BindingUsage(binding_usage) => binding_usage.eval(env),
            ExprKind::StringLiteral(string_literal) => Ok(string_literal.eval()),
            ExprKind::NumberLiteral(digits) => Ok(digits.eval()),
        }
    }
}

impl FunctionCall {
    fn eval(&self, env: &Env<'_>) -> Result<Val, EvalError> {
        // TODO: Add proper error handling for when function is not a lambda.

        let val = env
            .get_binding(&self.name().unwrap())
            .ok_or(EvalError::BindingDoesNotExist)?;

        match val {
            Val::Lambda(lambda) => {
                let params: Result<Vec<_>, _> = self
                    .params()
                    .unwrap()
                    .map(|param| param.eval(env))
                    .collect();

                let params = params?;

                lambda.eval(params.into_iter(), env)
            }
            _ => unreachable!(),
        }
    }
}

impl Lambda {
    fn eval(
        &self,
        params: impl ExactSizeIterator<Item = Val>,
        env: &Env<'_>,
    ) -> Result<Val, EvalError> {
        let mut new_env = env.create_child();

        if params.len() > self.param_names().unwrap().count() {
            return Err(EvalError::TooManyParams);
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
            .ok_or(EvalError::BindingDoesNotExist)
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
    use crate::parser::expr::{parse_binding_usage, parse_lambda};
    use crate::parser::Parser;

    #[test]
    fn evaluate_non_existent_binding_usage() {
        let mut p = Parser::new("$test");
        parse_binding_usage(&mut p);

        let syntax_node = p.finish_and_get_syntax();
        let binding_usage = BindingUsage::cast(syntax_node).unwrap();

        let env = Env::new();

        assert_eq!(
            binding_usage.eval(&env),
            Err(EvalError::BindingDoesNotExist)
        );
    }

    #[test]
    fn evaluate_binding_usage_that_does_exist() {
        let mut p = Parser::new("$foo-bar");
        parse_binding_usage(&mut p);

        let syntax_node = p.finish_and_get_syntax();
        let binding_usage = BindingUsage::cast(syntax_node).unwrap();

        let mut env = Env::new();
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

        let env = Env::new();

        // Applying id lambda to "hello" gives "hello".
        assert_eq!(
            apply_a_to_b_lambda.eval(
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

        let env = Env::new();

        assert_eq!(
            id_lambda.eval(
                vec![Val::Number(5), Val::Str("test".to_string())].into_iter(),
                &env,
            ),
            Err(EvalError::TooManyParams),
        );
    }
}
