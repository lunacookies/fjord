mod eval;

use crate::lexer::SyntaxKind;
use crate::{SyntaxElement, SyntaxNode, SyntaxToken};
use smol_str::SmolStr;

macro_rules! ast_node {
    ($node:ident, $kind:expr) => {
        #[derive(Clone)]
        #[allow(unused)]
        pub(crate) struct $node(SyntaxNode);

        impl $node {
            #[allow(unused)]
            fn cast(node: SyntaxNode) -> Option<Self> {
                if node.kind() == $kind {
                    Some(Self(node))
                } else {
                    None
                }
            }
        }
    };
}

ast_node!(Root, SyntaxKind::Root);

impl Root {
    fn items(&self) -> impl Iterator<Item = Item> {
        self.0.children().filter_map(Item::cast)
    }
}

struct Item(SyntaxNode);

enum ItemKind {
    Statement(BindingDef),
    Expr(Expr),
}

impl Item {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if BindingDef::cast(node.clone()).is_some() || Expr::cast(node.clone().into()).is_some() {
            Some(Self(node))
        } else {
            None
        }
    }

    fn kind(&self) -> ItemKind {
        BindingDef::cast(self.0.clone())
            .map(ItemKind::Statement)
            .or_else(|| Expr::cast(self.0.clone().into()).map(ItemKind::Expr))
            .unwrap()
    }
}

ast_node!(BindingDef, SyntaxKind::BindingDef);

impl BindingDef {
    fn binding_name(&self) -> Option<SmolStr> {
        self.0
            .children_with_tokens()
            .filter_map(|element| element.into_token())
            .find(|token| token.kind() == SyntaxKind::Atom)
            .map(|token| token.text().clone())
    }

    fn expr(&self) -> Option<Expr> {
        self.0
            .children()
            .filter_map(|node| Expr::cast(node.into()))
            .next()
    }
}

struct Expr(SyntaxElement);

enum ExprKind {
    FunctionCall(FunctionCall),
    Lambda(Lambda),
    BindingUsage(BindingUsage),
    StringLiteral(StringLiteral),
    NumberLiteral(Digits),
}

impl Expr {
    fn cast(element: SyntaxElement) -> Option<Self> {
        let is_expr = match element {
            SyntaxElement::Node(ref node) => {
                FunctionCall::cast(node.clone()).is_some()
                    || Lambda::cast(node.clone()).is_some()
                    || BindingUsage::cast(node.clone()).is_some()
            }
            SyntaxElement::Token(ref token) => {
                token.kind() == SyntaxKind::StringLiteral || token.kind() == SyntaxKind::Digits
            }
        };

        if is_expr {
            Some(Self(element))
        } else {
            None
        }
    }

    fn kind(&self) -> ExprKind {
        match &self.0 {
            SyntaxElement::Node(node) => FunctionCall::cast(node.clone())
                .map(ExprKind::FunctionCall)
                .or_else(|| Lambda::cast(node.clone()).map(ExprKind::Lambda))
                .or_else(|| BindingUsage::cast(node.clone()).map(ExprKind::BindingUsage))
                .unwrap(),
            SyntaxElement::Token(token) => StringLiteral::cast(token.clone())
                .map(ExprKind::StringLiteral)
                .or_else(|| Digits::cast(token.clone()).map(ExprKind::NumberLiteral))
                .unwrap(),
        }
    }
}

ast_node!(FunctionCall, SyntaxKind::FunctionCall);

ast_node!(FunctionCallParams, SyntaxKind::FunctionCallParams);

ast_node!(Lambda, SyntaxKind::Lambda);

ast_node!(LambdaParams, SyntaxKind::LambdaParams);

ast_node!(BindingUsage, SyntaxKind::BindingUsage);

impl BindingUsage {
    fn binding_name(&self) -> Option<SmolStr> {
        self.0
            .children_with_tokens()
            .filter_map(|element| element.into_token())
            .find(|token| token.kind() == SyntaxKind::Atom)
            .map(|token| token.text().clone())
    }
}

macro_rules! ast_token {
    ($token:ident, $kind:expr) => {
        #[derive(Clone)]
        #[allow(unused)]
        pub(crate) struct $token(SyntaxToken);

        impl $token {
            #[allow(unused)]
            fn cast(token: SyntaxToken) -> Option<Self> {
                if token.kind() == $kind {
                    Some(Self(token))
                } else {
                    None
                }
            }

            #[allow(unused)]
            fn text(&self) -> &SmolStr {
                self.0.text()
            }
        }
    };
}

ast_token!(Digits, SyntaxKind::Digits);

ast_token!(StringLiteral, SyntaxKind::StringLiteral);
