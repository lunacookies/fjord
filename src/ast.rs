use crate::lexer::SyntaxKind;
use crate::SyntaxNode;

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

ast_node!(Item, SyntaxKind::Item);

ast_node!(Expr, SyntaxKind::Expr);

ast_node!(Statement, SyntaxKind::Statement);

ast_node!(BindingDef, SyntaxKind::BindingDef);

ast_node!(FunctionCall, SyntaxKind::FunctionCall);

ast_node!(FunctionCallParams, SyntaxKind::FunctionCallParams);

ast_node!(Lambda, SyntaxKind::Lambda);

ast_node!(LambdaParams, SyntaxKind::LambdaParams);
