use super::{
    BindingDef, BindingUsage, Digits, Expr, ExprKind, FunctionCall, Item, ItemKind, Lambda, Root,
    StringLiteral,
};
use crate::env::Env;
use crate::val::Val;

impl Root {
    fn eval(&self, env: &mut Env) -> Val {
        for item in self.items() {
            item.eval(env);
        }

        todo!()
    }
}

impl Item {
    fn eval(&self, env: &mut Env) -> Val {
        match self.kind() {
            ItemKind::Statement(binding_def) => {
                binding_def.eval(env);
                Val::Nil
            }
            ItemKind::Expr(expr) => expr.eval(env),
        }
    }
}

impl BindingDef {
    fn eval(&self, env: &mut Env) {
        env.store_binding(self.binding_name().unwrap(), self.expr().unwrap().eval(env))
    }
}

impl Expr {
    fn eval(&self, env: &Env) -> Val {
        match self.kind() {
            ExprKind::FunctionCall(function_call) => function_call.eval(env),
            ExprKind::Lambda(lambda) => lambda.eval(env),
            ExprKind::BindingUsage(binding_usage) => binding_usage.eval(env),
            ExprKind::StringLiteral(string_literal) => string_literal.eval(),
            ExprKind::NumberLiteral(digits) => digits.eval(),
        }
    }
}

impl FunctionCall {
    fn eval(&self, env: &Env) -> Val {
        todo!()
    }
}

impl Lambda {
    fn eval(&self, env: &Env) -> Val {
        todo!()
    }
}

impl BindingUsage {
    fn eval(&self, env: &Env) -> Val {
        todo!()
    }
}

impl StringLiteral {
    fn eval(&self) -> Val {
        todo!()
    }
}

impl Digits {
    fn eval(&self) -> Val {
        todo!()
    }
}
