use crate::lexer::SyntaxKind;
use crate::{Op, SyntaxElement, SyntaxNode, SyntaxToken};
use rowan::NodeOrToken;
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
    BindingDef(BindingDef),
    Expr(Expr),
}

impl Item {
    fn cast(element: SyntaxElement) -> Option<Self> {
        if element.clone().into_node().map(BindingDef::cast).is_some()
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
            .and_then(BindingDef::cast)
            .map(ItemKind::BindingDef)
            .or_else(|| Expr::cast(self.0.clone()).map(ItemKind::Expr))
            .unwrap()
    }
}

ast_node!(BindingDef, SyntaxKind::BindingDef);

impl BindingDef {
    pub(crate) fn binding_name(&self) -> Option<SmolStr> {
        self.0
            .children_with_tokens()
            .filter_map(NodeOrToken::into_token)
            .find(|token| token.kind() == SyntaxKind::Atom)
            .map(|token| token.text().clone())
    }

    pub(crate) fn expr(&self) -> Option<Expr> {
        let mut children = self.0.children_with_tokens();

        loop {
            let element = children.next()?;

            if element.into_token().and_then(Equals::cast).is_some() {
                return children.find_map(Expr::cast);
            }
        }
    }
}

pub(crate) struct Expr(SyntaxElement);

pub(crate) enum ExprKind {
    BinOp(BinOp),
    If(If),
    FunctionCall(FunctionCall),
    Lambda(Lambda),
    BindingUsage(BindingUsage),
    Block(Block),
    Atom(Atom),
    NumberLiteral(Digits),
    StringLiteral(StringLiteral),
    True(True),
    False(False),
}

impl Expr {
    fn cast(element: SyntaxElement) -> Option<Self> {
        let is_expr = match element {
            SyntaxElement::Node(ref node) => {
                BinOp::cast(node.clone()).is_some()
                    || If::cast(node.clone()).is_some()
                    || FunctionCall::cast(node.clone()).is_some()
                    || Lambda::cast(node.clone()).is_some()
                    || BindingUsage::cast(node.clone()).is_some()
                    || Block::cast(node.clone()).is_some()
            }
            SyntaxElement::Token(ref token) => {
                token.kind() == SyntaxKind::Atom
                    || token.kind() == SyntaxKind::Digits
                    || token.kind() == SyntaxKind::StringLiteral
                    || token.kind() == SyntaxKind::True
                    || token.kind() == SyntaxKind::False
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
            SyntaxElement::Node(node) => BinOp::cast(node.clone())
                .map(ExprKind::BinOp)
                .or_else(|| If::cast(node.clone()).map(ExprKind::If))
                .or_else(|| FunctionCall::cast(node.clone()).map(ExprKind::FunctionCall))
                .or_else(|| Lambda::cast(node.clone()).map(ExprKind::Lambda))
                .or_else(|| BindingUsage::cast(node.clone()).map(ExprKind::BindingUsage))
                .or_else(|| Block::cast(node.clone()).map(ExprKind::Block))
                .unwrap(),
            SyntaxElement::Token(token) => Atom::cast(token.clone())
                .map(ExprKind::Atom)
                .or_else(|| Digits::cast(token.clone()).map(ExprKind::NumberLiteral))
                .or_else(|| StringLiteral::cast(token.clone()).map(ExprKind::StringLiteral))
                .or_else(|| True::cast(token.clone()).map(ExprKind::True))
                .or_else(|| False::cast(token.clone()).map(ExprKind::False))
                .unwrap(),
        }
    }

    pub(crate) fn text_range(&self) -> TextRange {
        self.0.text_range()
    }
}

ast_node!(BinOp, SyntaxKind::BinOp);

impl BinOp {
    pub(crate) fn op(&self) -> Option<OpToken> {
        self.0
            .children_with_tokens()
            .filter_map(NodeOrToken::into_token)
            .find_map(OpToken::cast)
    }

    pub(crate) fn lhs(&self) -> Option<Expr> {
        self.0.children_with_tokens().find_map(Expr::cast)
    }

    pub(crate) fn rhs(&self) -> Option<Expr> {
        self.0.children_with_tokens().filter_map(Expr::cast).nth(1)
    }
}

ast_node!(If, SyntaxKind::If);

impl If {
    pub(crate) fn condition(&self) -> Option<Expr> {
        self.0.children_with_tokens().find_map(Expr::cast)
    }

    pub(crate) fn true_branch(&self) -> Option<Expr> {
        self.0
            .children()
            .find_map(Block::cast)
            .and_then(|block| Expr::cast(block.0.into()))
    }

    pub(crate) fn false_branch(&self) -> Option<Expr> {
        self.0
            .children()
            .filter_map(Block::cast)
            .nth(1)
            .and_then(|block| Expr::cast(block.0.into()))
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
                .filter_map(NodeOrToken::into_token)
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
            .filter_map(NodeOrToken::into_token)
            .find(|token| token.kind() == SyntaxKind::Atom)
            .map(|token| token.text().clone())
    }
}

ast_node!(Block, SyntaxKind::Block);

impl Block {
    pub(crate) fn items(&self) -> impl Iterator<Item = Item> {
        self.0.children_with_tokens().filter_map(Item::cast)
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

            #[allow(unused)]
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

ast_token!(True, SyntaxKind::True);

ast_token!(False, SyntaxKind::False);

ast_token!(Equals, SyntaxKind::Equals);

pub(crate) struct OpToken(SyntaxToken);

impl OpToken {
    fn cast(token: SyntaxToken) -> Option<Self> {
        if Plus::cast(token.clone()).is_some()
            || Minus::cast(token.clone()).is_some()
            || Star::cast(token.clone()).is_some()
            || Slash::cast(token.clone()).is_some()
        {
            Some(Self(token))
        } else {
            None
        }
    }

    pub(crate) fn as_op(&self) -> Option<Op> {
        Plus::cast(self.0.clone())
            .map(|_| Op::Add)
            .or_else(|| Minus::cast(self.0.clone()).map(|_| Op::Sub))
            .or_else(|| Star::cast(self.0.clone()).map(|_| Op::Mul))
            .or_else(|| Slash::cast(self.0.clone()).map(|_| Op::Div))
    }
}

ast_token!(Plus, SyntaxKind::Plus);

ast_token!(Minus, SyntaxKind::Minus);

ast_token!(Star, SyntaxKind::Star);

ast_token!(Slash, SyntaxKind::Slash);
