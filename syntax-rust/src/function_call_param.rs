use nom::{
    bytes::complete::tag,
    combinator::{map, opt},
    sequence::pair,
};

#[derive(Debug, PartialEq)]
pub(crate) struct FunctionCallParam<'text> {
    val: crate::Expr<'text>,
    comma: Option<&'text str>,
}

impl<'text> FunctionCallParam<'text> {
    pub(crate) fn new(s: &'text str) -> nom::IResult<&'text str, Self> {
        map(pair(crate::Expr::new, opt(tag(","))), |(val, comma)| Self {
            val,
            comma,
        })(s)
    }
}

impl<'param> From<FunctionCallParam<'param>> for Vec<syntax::HighlightedSpan<'param>> {
    fn from(param: FunctionCallParam<'param>) -> Self {
        let mut output = Vec::from(param.val);

        if let Some(comma) = param.comma {
            output.push(syntax::HighlightedSpan {
                text: comma,
                group: Some(syntax::HighlightGroup::Separator),
            });
        }

        output
    }
}
