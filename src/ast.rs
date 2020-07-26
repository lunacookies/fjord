use crate::lexer::SyntaxKind;
use crate::SyntaxNode;
use smol_str::SmolStr;

macro_rules! ast_node {
    ($node:ident, $kind:expr) => {
        #[allow(unused)]
        struct $node(SyntaxNode);

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
    Statement(Statement),
    Expr(Expr),
}

impl Item {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if Statement::cast(node.clone()).is_some() || Expr::cast(node.clone()).is_some() {
            Some(Self(node))
        } else {
            None
        }
    }

    fn kind(&self) -> ItemKind {
        Statement::cast(self.0.clone())
            .map(ItemKind::Statement)
            .or_else(|| Expr::cast(self.0.clone()).map(ItemKind::Expr))
            .unwrap()
    }
}

ast_node!(Statement, SyntaxKind::Statement);

impl Statement {
    fn binding_name(&self) -> Option<&SmolStr> {
        self.0
            .children_with_tokens()
            .filter_map(|element| element.into_token())
            .filter(|token| token.kind() == SyntaxKind::Atom)
            .next()
            .map(|token| token.text())
    }
}

ast_node!(BindingDef, SyntaxKind::BindingDef);

ast_node!(Expr, SyntaxKind::Expr);

ast_node!(FunctionCall, SyntaxKind::FunctionCall);

ast_node!(FunctionCallParams, SyntaxKind::FunctionCallParams);

ast_node!(Lambda, SyntaxKind::Lambda);

ast_node!(LambdaParams, SyntaxKind::LambdaParams);
