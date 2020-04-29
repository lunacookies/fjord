mod named_structure_def_fields;
mod tuple_structure_def_fields;

use {
    crate::utils::{
        ident, {take_whitespace0, take_whitespace1},
    },
    named_structure_def_fields::fields as named_structure_def_fields,
    nom::{
        branch::alt,
        bytes::complete::{tag, take, take_till1, take_until},
        combinator::{map, not, opt},
        multi::many0,
        sequence::pair,
    },
    tuple_structure_def_fields::fields as tuple_structure_def_fields,
};

type ParseResult<'text> = nom::IResult<&'text str, Vec<syntax::HighlightedSpan<'text>>>;

fn expect<'input>(
    parser: impl Fn(&'input str) -> ParseResult<'input> + Copy,
) -> impl Fn(&'input str) -> ParseResult<'input> {
    move |s| {
        if let Ok((s, spans)) = parser(s) {
            Ok((s, spans))
        } else {
            // If the input parser fails, then take a single character as an error and attempt
            // parsing again.
            let (s, error) = error_1_char(s)?;
            let (s, mut parser_output) = expect(parser)(s)?;

            let mut output = error;
            output.append(&mut parser_output);

            Ok((s, output))
        }
    }
}

fn comma_separated<'input, P: Fn(&'input str) -> ParseResult<'input> + Copy + 'input>(
    parser: &'input P,
) -> impl Fn(&'input str) -> ParseResult<'input> + 'input {
    let preceded_by_comma = move |s| {
        let (s, initial_space) = take_whitespace0(s)?;

        let (s, mut comma) = expect(|s| {
            map(tag(","), |s| {
                vec![syntax::HighlightedSpan {
                    text: s,
                    group: Some(syntax::HighlightGroup::Separator),
                }]
            })(s)
        })(s)?;

        let (s, comma_space) = take_whitespace0(s)?;

        let (s, mut parser_output) = expect(parser)(s)?;

        let mut output = vec![syntax::HighlightedSpan {
            text: initial_space,
            group: None,
        }];

        output.append(&mut comma);

        output.push(syntax::HighlightedSpan {
            text: comma_space,
            group: None,
        });

        output.append(&mut parser_output);

        Ok((s, output))
    };

    move |s| {
        let (s, first) = match parser(s) {
            Ok((s, first)) => (s, first),
            _ => return Ok((s, vec![])),
        };

        let (s, rest) = many0(preceded_by_comma)(s)?;

        let (s, space) = take_whitespace0(s)?;
        let (s, trailing_comma) = opt(tag(","))(s)?;

        let mut output = first;
        output.append(&mut rest.concat());

        output.push(syntax::HighlightedSpan {
            text: space,
            group: None,
        });

        if let Some(trailing_comma) = trailing_comma {
            output.push(syntax::HighlightedSpan {
                text: trailing_comma,
                group: Some(syntax::HighlightGroup::Separator),
            });
        }

        Ok((s, output))
    }
}

pub(crate) fn parse(s: &str) -> ParseResult<'_> {
    alt((item, whitespace, error))(s)
}

fn item(s: &str) -> ParseResult<'_> {
    alt((function, structure_def))(s)
}

fn whitespace(s: &str) -> ParseResult<'_> {
    map(take_whitespace1, |s| {
        vec![syntax::HighlightedSpan {
            text: s,
            group: None,
        }]
    })(s)
}

fn error(s: &str) -> ParseResult<'_> {
    map(
        alt((
            // ‘Reset’ errors after any of these characters.
            take_till1(|c| c == '}' || c == ';'),
            // This will fail, however, if the input starts with any of these ‘reset’
            // characters. In that case, we simply take a single character.
            take(1usize),
        )),
        |s| {
            vec![syntax::HighlightedSpan {
                text: s,
                group: Some(syntax::HighlightGroup::Error),
            }]
        },
    )(s)
}

fn error_1_char(s: &str) -> ParseResult<'_> {
    map(take(1usize), |s| {
        vec![syntax::HighlightedSpan {
            text: s,
            group: Some(syntax::HighlightGroup::Error),
        }]
    })(s)
}

fn function(s: &str) -> ParseResult<'_> {
    let (s, keyword) = tag("fn")(s)?;
    let (s, keyword_space) = take_whitespace1(s)?;

    let (s, name) = ident(s)?;
    let (s, name_space) = take_whitespace0(s)?;

    let (s, open_paren) = tag("(")(s)?;
    let (s, open_paren_space) = take_whitespace0(s)?;

    let (s, close_paren) = tag(")")(s)?;
    let (s, close_paren_space) = take_whitespace0(s)?;

    let (s, return_type) = opt(function_return_type)(s)?;
    let (s, return_type_space) = take_whitespace0(s)?;

    let (s, mut body) = block(s)?;

    let mut output = vec![
        syntax::HighlightedSpan {
            text: keyword,
            group: Some(syntax::HighlightGroup::OtherKeyword),
        },
        syntax::HighlightedSpan {
            text: keyword_space,
            group: None,
        },
        syntax::HighlightedSpan {
            text: name,
            group: Some(syntax::HighlightGroup::FunctionDef),
        },
        syntax::HighlightedSpan {
            text: name_space,
            group: None,
        },
        syntax::HighlightedSpan {
            text: open_paren,
            group: Some(syntax::HighlightGroup::Delimiter),
        },
        syntax::HighlightedSpan {
            text: open_paren_space,
            group: None,
        },
        syntax::HighlightedSpan {
            text: close_paren,
            group: Some(syntax::HighlightGroup::Delimiter),
        },
        syntax::HighlightedSpan {
            text: close_paren_space,
            group: None,
        },
    ];

    if let Some(mut return_type) = return_type {
        output.append(&mut return_type);
    }

    output.push(syntax::HighlightedSpan {
        text: return_type_space,
        group: None,
    });

    output.append(&mut body);

    Ok((s, output))
}

fn function_return_type(s: &str) -> ParseResult<'_> {
    let (s, arrow) = tag("->")(s)?;
    let (s, arrow_space) = take_whitespace0(s)?;

    let (s, mut ty) = ty(s)?;

    let mut output = vec![
        syntax::HighlightedSpan {
            text: arrow,
            group: Some(syntax::HighlightGroup::Separator),
        },
        syntax::HighlightedSpan {
            text: arrow_space,
            group: None,
        },
    ];

    output.append(&mut ty);

    Ok((s, output))
}

fn structure_def(s: &str) -> ParseResult<'_> {
    let (s, keyword) = tag("struct")(s)?;
    let (s, keyword_space) = take_whitespace1(s)?;

    let (s, name) = ident(s)?;
    let (s, name_space) = take_whitespace0(s)?;

    let (s, mut fields) = structure_def_fields(s)?;

    let mut output = vec![
        syntax::HighlightedSpan {
            text: keyword,
            group: Some(syntax::HighlightGroup::OtherKeyword),
        },
        syntax::HighlightedSpan {
            text: keyword_space,
            group: None,
        },
        syntax::HighlightedSpan {
            text: name,
            group: Some(syntax::HighlightGroup::TyDef),
        },
        syntax::HighlightedSpan {
            text: name_space,
            group: None,
        },
    ];

    output.append(&mut fields);

    Ok((s, output))
}

fn structure_def_fields(s: &str) -> ParseResult<'_> {
    alt((
        named_structure_def_fields,
        tuple_structure_def_fields,
        unnamed_structure,
    ))(s)
}

fn unnamed_structure(s: &str) -> ParseResult<'_> {
    map(tag(";"), |semicolon| {
        vec![syntax::HighlightedSpan {
            text: semicolon,
            group: Some(syntax::HighlightGroup::Terminator),
        }]
    })(s)
}

fn block(s: &str) -> ParseResult<'_> {
    let (s, open_brace) = tag("{")(s)?;
    let (s, open_brace_space) = take_whitespace0(s)?;

    let (s, statements) = many0(|s| {
        // Only continue parsing statements if we’re not at the end of the block.
        let end_block = pair(take_whitespace0, tag("}"));
        let _ = not(end_block)(s)?;

        let (s, statement) = expect(statement)(s)?;
        let (s, space) = take_whitespace0(s)?;

        let mut output = statement;
        output.push(syntax::HighlightedSpan {
            text: space,
            group: None,
        });

        Ok((s, output))
    })(s)?;

    let (s, close_brace_space) = take_whitespace0(s)?;
    let (s, close_brace) = tag("}")(s)?;

    let mut output = vec![
        syntax::HighlightedSpan {
            text: open_brace,
            group: Some(syntax::HighlightGroup::Delimiter),
        },
        syntax::HighlightedSpan {
            text: open_brace_space,
            group: None,
        },
    ];

    output.append(&mut statements.concat());

    output.extend_from_slice(&[
        syntax::HighlightedSpan {
            text: close_brace_space,
            group: None,
        },
        syntax::HighlightedSpan {
            text: close_brace,
            group: Some(syntax::HighlightGroup::Delimiter),
        },
    ]);

    Ok((s, output))
}

fn statement(s: &str) -> ParseResult<'_> {
    alt((item, let_statement, expr_in_statement))(s)
}

fn let_statement(s: &str) -> ParseResult<'_> {
    let (s, keyword) = tag("let")(s)?;
    let (s, keyword_space) = take_whitespace1(s)?;

    let (s, mut pattern) = pattern(s)?;
    let (s, pattern_space) = take_whitespace0(s)?;

    let (s, rhs) = opt(|s| {
        let (s, equals) = tag("=")(s)?;
        let (s, equals_space) = take_whitespace0(s)?;

        let (s, mut expr) = expr(s)?;
        let (s, expr_space) = take_whitespace0(s)?;

        let mut output = vec![
            syntax::HighlightedSpan {
                text: equals,
                group: Some(syntax::HighlightGroup::AssignOper),
            },
            syntax::HighlightedSpan {
                text: equals_space,
                group: None,
            },
        ];

        output.append(&mut expr);
        output.push(syntax::HighlightedSpan {
            text: expr_space,
            group: None,
        });

        Ok((s, output))
    })(s)?;

    let (s, semicolon) = tag(";")(s)?;

    let mut output = vec![
        syntax::HighlightedSpan {
            text: keyword,
            group: Some(syntax::HighlightGroup::OtherKeyword),
        },
        syntax::HighlightedSpan {
            text: keyword_space,
            group: None,
        },
    ];

    output.append(&mut pattern);
    output.push(syntax::HighlightedSpan {
        text: pattern_space,
        group: None,
    });

    if let Some(mut rhs) = rhs {
        output.append(&mut rhs);
    }

    output.push(syntax::HighlightedSpan {
        text: semicolon,
        group: Some(syntax::HighlightGroup::Terminator),
    });

    Ok((s, output))
}

fn expr_in_statement(s: &str) -> ParseResult<'_> {
    let (s, expr) = expr(s)?;
    let (s, expr_space) = take_whitespace0(s)?;

    let (s, semicolon) = opt(tag(";"))(s)?;

    let mut output = expr;
    output.push(syntax::HighlightedSpan {
        text: expr_space,
        group: None,
    });

    if let Some(semicolon) = semicolon {
        output.push(syntax::HighlightedSpan {
            text: semicolon,
            group: Some(syntax::HighlightGroup::Terminator),
        });
    }

    Ok((s, output))
}

fn expr(s: &str) -> ParseResult<'_> {
    alt((function_call, variable, string))(s)
}

fn function_call(s: &str) -> ParseResult<'_> {
    let (s, name) = ident(s)?;
    let (s, name_space) = take_whitespace0(s)?;

    let (s, open_paren) = tag("(")(s)?;
    let (s, open_paren_space) = take_whitespace0(s)?;

    // Function calls take in expressions.
    let (s, mut params) = comma_separated(&expr)(s)?;
    let (s, params_space) = take_whitespace0(s)?;

    let (s, close_paren) = tag(")")(s)?;

    let mut output = vec![
        syntax::HighlightedSpan {
            text: name,
            group: Some(syntax::HighlightGroup::FunctionCall),
        },
        syntax::HighlightedSpan {
            text: name_space,
            group: None,
        },
        syntax::HighlightedSpan {
            text: open_paren,
            group: Some(syntax::HighlightGroup::Delimiter),
        },
        syntax::HighlightedSpan {
            text: open_paren_space,
            group: None,
        },
    ];

    output.append(&mut params);

    output.extend_from_slice(&[
        syntax::HighlightedSpan {
            text: params_space,
            group: None,
        },
        syntax::HighlightedSpan {
            text: close_paren,
            group: Some(syntax::HighlightGroup::Delimiter),
        },
    ]);

    Ok((s, output))
}

fn variable(s: &str) -> ParseResult<'_> {
    map(ident, |s| {
        vec![syntax::HighlightedSpan {
            text: s,
            group: Some(syntax::HighlightGroup::VariableUse),
        }]
    })(s)
}

fn string(s: &str) -> ParseResult<'_> {
    let (s, start_quote) = tag("\"")(s)?;
    let (s, contents) = take_until("\"")(s)?;
    let (s, end_quote) = tag("\"")(s)?;

    let output = vec![
        syntax::HighlightedSpan {
            text: start_quote,
            group: Some(syntax::HighlightGroup::StringDelimiter),
        },
        syntax::HighlightedSpan {
            text: contents,
            group: Some(syntax::HighlightGroup::String),
        },
        syntax::HighlightedSpan {
            text: end_quote,
            group: Some(syntax::HighlightGroup::StringDelimiter),
        },
    ];

    Ok((s, output))
}

fn pattern(s: &str) -> ParseResult<'_> {
    map(ident, |s| {
        vec![syntax::HighlightedSpan {
            text: s,
            group: Some(syntax::HighlightGroup::VariableDef),
        }]
    })(s)
}

fn ty(s: &str) -> ParseResult<'_> {
    map(ident, |s| {
        vec![syntax::HighlightedSpan {
            text: s,
            group: Some(syntax::HighlightGroup::TyUse),
        }]
    })(s)
}
