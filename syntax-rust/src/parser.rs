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
    },
    tuple_structure_def_fields::fields as tuple_structure_def_fields,
};

type ParseResult<'text> = nom::IResult<&'text str, Vec<syntax::HighlightedSpan<'text>>>;

fn expect<'input>(
    parser: impl Fn(&'input str) -> ParseResult<'input> + Copy,
    ending_sequence: Option<&'input str>,
) -> impl Fn(&'input str) -> ParseResult<'input> {
    move |s| {
        if let Ok((s, spans)) = parser(s) {
            Ok((s, spans))
        } else {
            // Stop parsing if the user has chosen an ending sequence and if we’ve reached the end
            // of this sequence.
            if let Some(ending_sequence) = ending_sequence {
                let _ = not(tag(ending_sequence))(s)?;
            }

            // If the input parser fails, then take a single character as an error and attempt
            // parsing again.
            let (s, error) = error_1_char(s)?;
            let (s, mut parser_output) = expect(parser, ending_sequence)(s)?;

            let mut output = error;
            output.append(&mut parser_output);

            Ok((s, output))
        }
    }
}

fn comma_separated<'input, P: Fn(&'input str) -> ParseResult<'input> + Copy + 'input>(
    parser: &'input P,
    ending_sequence: &'input str,
) -> impl Fn(&'input str) -> ParseResult<'input> + 'input {
    let parser_stop_at_end = move |s| expect(parser, Some(ending_sequence))(s);

    let comma_stop_at_end = move |s| {
        expect(
            |s| {
                map(tag(","), |s| {
                    vec![syntax::HighlightedSpan {
                        text: s,
                        group: Some(syntax::HighlightGroup::Separator),
                    }]
                })(s)
            },
            Some(ending_sequence),
        )(s)
    };

    let followed_by_comma = move |s| {
        let (s, parser_output) = parser_stop_at_end(s)?;
        let (s, parser_output_space) = take_whitespace0(s)?;

        let (s, mut comma) = comma_stop_at_end(s)?;
        let (s, comma_space) = take_whitespace0(s)?;

        let mut output = parser_output;

        output.push(syntax::HighlightedSpan {
            text: parser_output_space,
            group: None,
        });

        output.append(&mut comma);

        output.push(syntax::HighlightedSpan {
            text: comma_space,
            group: None,
        });

        Ok((s, output))
    };

    move |s| {
        let (s, followed_by_commas) = many0(followed_by_comma)(s)?;
        let (s, last_without_comma) = opt(parser_stop_at_end)(s)?;

        let (s, space) = take_whitespace0(s)?;

        let mut output = followed_by_commas.concat();

        if let Some(mut last_without_comma) = last_without_comma {
            output.append(&mut last_without_comma);
        }

        output.push(syntax::HighlightedSpan {
            text: space,
            group: None,
        });

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

fn type_alias(s: &str) -> ParseResult<'_> {
    let (s, keyword) = tag("type")(s)?;
    let (s, keyword_space) = take_whitespace1(s)?;

    let (s, name) = ident(s)?;
    let (s, name_space) = take_whitespace0(s)?;

    let (s, equals) = tag("=")(s)?;
    let (s, equals_space) = take_whitespace0(s)?;

    let (s, mut ty) = ty(s)?;
    let (s, ty_space) = take_whitespace0(s)?;

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
        syntax::HighlightedSpan {
            text: name,
            group: Some(syntax::HighlightGroup::TyDef),
        },
        syntax::HighlightedSpan {
            text: name_space,
            group: None,
        },
        syntax::HighlightedSpan {
            text: equals,
            group: Some(syntax::HighlightGroup::AssignOper),
        },
        syntax::HighlightedSpan {
            text: equals_space,
            group: None,
        },
    ];

    output.append(&mut ty);

    output.extend_from_slice(&[
        syntax::HighlightedSpan {
            text: ty_space,
            group: None,
        },
        syntax::HighlightedSpan {
            text: semicolon,
            group: Some(syntax::HighlightGroup::Terminator),
        },
    ]);

    Ok((s, output))
}

fn function(s: &str) -> ParseResult<'_> {
    let start_params = "(";
    let end_params = ")";

    let (s, keyword) = tag("fn")(s)?;
    let (s, keyword_space) = take_whitespace1(s)?;

    let (s, name) = ident(s)?;
    let (s, name_space) = take_whitespace0(s)?;

    let (s, open_paren) = tag(start_params)(s)?;
    let (s, open_paren_space) = take_whitespace0(s)?;

    let (s, mut params) = comma_separated(&function_def_param, end_params)(s)?;

    let (s, close_paren) = tag(end_params)(s)?;
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
    ];

    output.append(&mut params);

    output.extend_from_slice(&[
        syntax::HighlightedSpan {
            text: close_paren,
            group: Some(syntax::HighlightGroup::Delimiter),
        },
        syntax::HighlightedSpan {
            text: close_paren_space,
            group: None,
        },
    ]);

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

fn function_def_param(s: &str) -> ParseResult<'_> {
    let (s, name) = ident(s)?;
    let (s, name_space) = take_whitespace0(s)?;

    let (s, colon) = tag(":")(s)?;
    let (s, colon_space) = take_whitespace0(s)?;

    let (s, mut ty) = ty(s)?;

    let mut output = vec![
        syntax::HighlightedSpan {
            text: name,
            group: Some(syntax::HighlightGroup::FunctionParam),
        },
        syntax::HighlightedSpan {
            text: name_space,
            group: None,
        },
        syntax::HighlightedSpan {
            text: colon,
            group: Some(syntax::HighlightGroup::Separator),
        },
        syntax::HighlightedSpan {
            text: colon_space,
            group: None,
        },
    ];

    output.append(&mut ty);

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
    let start_block = "{";
    let end_block = "}";

    let (s, open_brace) = tag(start_block)(s)?;
    let (s, open_brace_space) = take_whitespace0(s)?;

    let (s, statements) = many0(|s| {
        let (s, statement) = expect(statement, Some(end_block))(s)?;
        let (s, space) = take_whitespace0(s)?;

        let mut output = statement;
        output.push(syntax::HighlightedSpan {
            text: space,
            group: None,
        });

        Ok((s, output))
    })(s)?;

    let (s, close_brace_space) = take_whitespace0(s)?;
    let (s, close_brace) = tag(end_block)(s)?;

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

        let (s, mut expr) = expect(expr, None)(s)?;
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

    let (s, mut semicolon) = expect(
        |s| {
            map(tag(";"), |s| {
                vec![syntax::HighlightedSpan {
                    text: s,
                    group: Some(syntax::HighlightGroup::Terminator),
                }]
            })(s)
        },
        None,
    )(s)?;

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

    output.append(&mut semicolon);

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
    let (s, expr) = alt((function_call, variable, string))(s)?;

    // ‘follower’ is the term I’ve given to method calls and field accesses.
    let (s, followers) = many0(alt((method_call, field_access)))(s)?;

    let mut output = expr;
    output.append(&mut followers.concat());

    Ok((s, output))
}

fn method_call(s: &str) -> ParseResult<'_> {
    let (s, period) = tag(".")(s)?;
    let (s, period_space) = take_whitespace0(s)?;

    let (s, mut function_call) = function_call(s)?;

    let mut output = vec![
        syntax::HighlightedSpan {
            text: period,
            group: Some(syntax::HighlightGroup::MemberOper),
        },
        syntax::HighlightedSpan {
            text: period_space,
            group: None,
        },
    ];

    output.append(&mut function_call);

    Ok((s, output))
}

fn field_access(s: &str) -> ParseResult<'_> {
    let (s, period) = tag(".")(s)?;
    let (s, period_space) = take_whitespace0(s)?;

    let (s, field) = ident(s)?;

    let output = vec![
        syntax::HighlightedSpan {
            text: period,
            group: Some(syntax::HighlightGroup::MemberOper),
        },
        syntax::HighlightedSpan {
            text: period_space,
            group: None,
        },
        syntax::HighlightedSpan {
            text: field,
            group: Some(syntax::HighlightGroup::MemberUse),
        },
    ];

    Ok((s, output))
}

fn function_call(s: &str) -> ParseResult<'_> {
    let (s, name) = ident(s)?;
    let (s, name_space) = take_whitespace0(s)?;

    let (s, open_paren) = tag("(")(s)?;
    let (s, open_paren_space) = take_whitespace0(s)?;

    // Function calls take in expressions.
    let (s, mut params) = comma_separated(&expr, ")")(s)?;
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
