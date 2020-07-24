use crate::lexer::{Lexer, SyntaxKind};
use crate::SyntaxNode;
use rowan::{GreenNode, GreenNodeBuilder};

/// The output of parsing Fjord code.
pub struct ParseOutput {
    green_node: GreenNode,
}

#[cfg(test)]
impl ParseOutput {
    fn syntax(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.green_node.clone())
    }

    fn debug_tree(&self) -> String {
        format!("{:#?}", self.syntax()).trim().to_string()
    }
}

/// Parses Fjord code.
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    builder: GreenNodeBuilder<'static>,
}

impl<'a> Parser<'a> {
    /// Creates a new Parser given the input.
    pub fn new(input: &'a str) -> Self {
        Self {
            lexer: Lexer::new(input),
            builder: GreenNodeBuilder::new(),
        }
    }

    /// Parses the input the `Parser` was constructed with.
    pub fn parse(mut self) -> ParseOutput {
        self.builder.start_node(SyntaxKind::Root.into());
        self.builder.finish_node();

        ParseOutput {
            green_node: self.builder.finish(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(input: &str, expected_output: &str) {
        let parse_output = Parser::new(input).parse();
        assert_eq!(parse_output.debug_tree(), expected_output.trim());
    }

    #[test]
    fn parse_nothing() {
        test(
            "",
            r#"
Root@0..0"#,
        );
    }
}
