use super::{
    BindingDef, BindingUsage, Digits, Expr, ExprKind, FunctionCall, Item, ItemKind, Lambda, Root,
    Statement, StatementKind, StringLiteral,
};
use crate::env::Env;
use crate::val::Val;

impl Root {
    pub(crate) fn eval(&self, env: &mut Env<'_>) -> Val {
        for item in self.items() {
            if let ItemKind::Statement(statement) = item.kind() {
                if let StatementKind::ReturnStatement(return_statement) = statement.kind() {
                    return return_statement
                        .val()
                        .map(|expr| expr.eval(env))
                        .unwrap_or(Val::Nil);
                }
            }

            item.eval(env);
        }

        Val::Nil
    }
}

impl Item {
    fn eval(&self, env: &mut Env<'_>) -> Val {
        match self.kind() {
            ItemKind::Statement(statement) => {
                statement.eval(env);
                Val::Nil
            }
            ItemKind::Expr(expr) => expr.eval(env),
        }
    }
}

impl Statement {
    fn eval(&self, env: &mut Env<'_>) {
        match self.kind() {
            StatementKind::BindingDef(binding_def) => binding_def.eval(env),
            StatementKind::ReturnStatement(_) => {}
        }
    }
}

impl BindingDef {
    fn eval(&self, env: &mut Env<'_>) {
        env.store_binding(self.binding_name().unwrap(), self.expr().unwrap().eval(env))
    }
}

impl Expr {
    fn eval(&self, env: &Env<'_>) -> Val {
        match self.kind() {
            ExprKind::FunctionCall(function_call) => function_call.eval(env),
            ExprKind::Lambda(lambda) => Val::Lambda(lambda),
            ExprKind::BindingUsage(binding_usage) => binding_usage.eval(env),
            ExprKind::StringLiteral(string_literal) => string_literal.eval(),
            ExprKind::NumberLiteral(digits) => digits.eval(),
        }
    }
}

impl FunctionCall {
    fn eval(&self, env: &Env<'_>) -> Val {
        // TODO: Add proper error handling for when function does not exist or is not a lambda.

        let val = env.get_binding(&self.name().unwrap()).unwrap();

        match val {
            Val::Lambda(lambda) => {
                lambda.eval(self.params().unwrap().map(|param| param.eval(env)), env)
            }
            _ => unreachable!(),
        }
    }
}

impl Lambda {
    fn eval(&self, params: impl Iterator<Item = Val>, env: &Env<'_>) -> Val {
        let mut new_env = env.create_child();

        for (param_name, param_val) in self.param_names().unwrap().zip(params) {
            new_env.store_binding(param_name, param_val);
        }

        self.body().unwrap().eval(&new_env)
    }
}

impl BindingUsage {
    fn eval(&self, env: &Env<'_>) -> Val {
        // TODO: Add proper error handling for if the binding does not exist.
        env.get_binding(&self.binding_name().unwrap()).unwrap()
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
