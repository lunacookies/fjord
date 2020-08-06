//! Abstract Syntax Trees.
//!
//! The nodes here are partially auto-generated, and as such lack documentation.

use crate::lexer::SyntaxKind;
use crate::{SyntaxElement, SyntaxNode, SyntaxToken};
use smol_str::SmolStr;
use text_size::TextRange;

macro_rules! ast_node {
    ($node:ident, $kind:expr) => {
        #[allow(missing_docs)]
        #[derive(Debug, Clone, Eq, PartialEq, Hash)]
        pub struct $node(SyntaxNode);

        impl $node {
            pub(crate) fn cast(node: SyntaxNode) -> Option<Self> {
                if node.kind() == $kind {
                    Some(Self(node))
                } else {
                    None
                }
            }

            #[allow(unused)]
            pub(crate) fn text_range(&self) -> TextRange {
                self.0.text_range()
            }
        }
    };
}

ast_node!(Root, SyntaxKind::Root);

impl Root {
    pub(crate) fn items(&self) -> impl Iterator<Item = Item> {
        self.0.children_with_tokens().filter_map(Item::cast)
    }
}

pub(crate) struct Item(SyntaxElement);

pub(crate) enum ItemKind {
    Statement(Statement),
    Expr(Expr),
}

impl Item {
    fn cast(element: SyntaxElement) -> Option<Self> {
        if element.clone().into_node().map(Statement::cast).is_some()
            || Expr::cast(element.clone()).is_some()
        {
            Some(Self(element))
        } else {
            None
        }
    }

    pub(crate) fn kind(&self) -> ItemKind {
        self.0
            .clone()
            .into_node()
            .and_then(Statement::cast)
            .map(ItemKind::Statement)
            .or_else(|| Expr::cast(self.0.clone()).map(ItemKind::Expr))
            .unwrap()
    }
}

pub(crate) struct Statement(SyntaxNode);

pub(crate) enum StatementKind {
    BindingDef(BindingDef),
    ReturnStatement(ReturnStatement),
}

impl Statement {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if BindingDef::cast(node.clone()).is_some() || ReturnStatement::cast(node.clone()).is_some()
        {
            Some(Self(node))
        } else {
            None
        }
    }

    pub(crate) fn kind(&self) -> StatementKind {
        BindingDef::cast(self.0.clone())
            .map(StatementKind::BindingDef)
            .or_else(|| ReturnStatement::cast(self.0.clone()).map(StatementKind::ReturnStatement))
            .unwrap()
    }
}

ast_node!(BindingDef, SyntaxKind::BindingDef);

impl BindingDef {
    pub(crate) fn binding_name(&self) -> Option<SmolStr> {
        self.0
            .children_with_tokens()
            .filter_map(|element| element.into_token())
            .find(|token| token.kind() == SyntaxKind::Atom)
            .map(|token| token.text().clone())
    }

    pub(crate) fn expr(&self) -> Option<Expr> {
        self.0.children_with_tokens().find_map(Expr::cast)
    }
}

ast_node!(ReturnStatement, SyntaxKind::ReturnStatement);

impl ReturnStatement {
    pub(crate) fn val(&self) -> Option<Expr> {
        self.0.children_with_tokens().find_map(Expr::cast)
    }
}

pub(crate) struct Expr(SyntaxElement);

pub(crate) enum ExprKind {
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

    pub(crate) fn kind(&self) -> ExprKind {
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

impl FunctionCall {
    pub(crate) fn name(&self) -> Option<Atom> {
        self.0.first_token().and_then(Atom::cast)
    }

    pub(crate) fn params(&self) -> Option<FunctionCallParams> {
        self.0.children().find_map(FunctionCallParams::cast)
    }

    pub(crate) fn param_exprs(&self) -> Option<impl Iterator<Item = Expr>> {
        self.params()
            .map(|params| params.0.children_with_tokens().filter_map(Expr::cast))
    }
}

ast_node!(FunctionCallParams, SyntaxKind::FunctionCallParams);

ast_node!(Lambda, SyntaxKind::Lambda);

impl Lambda {
    pub(crate) fn param_names(&self) -> Option<impl Iterator<Item = SmolStr>> {
        let params = LambdaParams::cast(self.0.first_child()?)?;

        Some(
            params
                .0
                .children_with_tokens()
                .filter_map(|element| element.into_token())
                .filter_map(Atom::cast)
                .map(|atom| atom.text().clone()),
        )
    }

    pub(crate) fn body(&self) -> Option<Expr> {
        self.0.children_with_tokens().find_map(Expr::cast)
    }
}

ast_node!(LambdaParams, SyntaxKind::LambdaParams);

ast_node!(BindingUsage, SyntaxKind::BindingUsage);

impl BindingUsage {
    pub(crate) fn binding_name(&self) -> Option<SmolStr> {
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
        pub(crate) struct $token(SyntaxToken);

        impl $token {
            pub(crate) fn cast(token: SyntaxToken) -> Option<Self> {
                if token.kind() == $kind {
                    Some(Self(token))
                } else {
                    None
                }
            }

            pub(crate) fn text(&self) -> &SmolStr {
                self.0.text()
            }

            #[allow(unused)]
            pub(crate) fn text_range(&self) -> TextRange {
                self.0.text_range()
            }
        }
    };
}

ast_token!(Atom, SyntaxKind::Atom);

ast_token!(Digits, SyntaxKind::Digits);

ast_token!(StringLiteral, SyntaxKind::StringLiteral);
