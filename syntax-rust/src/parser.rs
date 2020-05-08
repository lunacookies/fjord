mod named_structure_def_fields;
mod tuple_structure_def_fields;

use {
    crate::utils::{
        digits, float_ty, int_ty, pascal_case, snake_case, {take_whitespace0, take_whitespace1},
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
    alt((trait_, use_, ty_alias, function, structure_def))(s)
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

fn trait_(s: &str) -> ParseResult<'_> {
    let (s, keyword) = tag("trait")(s)?;
    let (s, keyword_space) = take_whitespace1(s)?;

    let (s, name) = pascal_case(s)?;
    let (s, name_space) = take_whitespace0(s)?;

    let (s, open_brace) = tag("{")(s)?;
    let (s, open_brace_space) = take_whitespace0(s)?;

    let (s, items) = many0(|s| {
        let (s, item) = item(s)?;
        let (s, space) = take_whitespace0(s)?;

        let mut output = item;
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
            text: keyword,
            group: Some(syntax::HighlightGroup::OtherKeyword),
        },
        syntax::HighlightedSpan {
            text: keyword_space,
            group: None,
        },
        syntax::HighlightedSpan {
            text: name,
            group: Some(syntax::HighlightGroup::InterfaceDef),
        },
        syntax::HighlightedSpan {
            text: name_space,
            group: None,
        },
        syntax::HighlightedSpan {
            text: open_brace,
            group: Some(syntax::HighlightGroup::Delimiter),
        },
        syntax::HighlightedSpan {
            text: open_brace_space,
            group: None,
        },
    ];

    output.append(&mut items.concat());

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

fn use_(s: &str) -> ParseResult<'_> {
    let (s, keyword) = tag("use")(s)?;
    let (s, keyword_space) = take_whitespace1(s)?;

    let (s, mut path) = path(s)?;
    let (s, path_space) = take_whitespace0(s)?;

    let (s, mut ident) = alt((ty_name, module_name))(s)?;
    let (s, ident_space) = take_whitespace0(s)?;

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

    output.append(&mut path);

    output.push(syntax::HighlightedSpan {
        text: path_space,
        group: None,
    });

    output.append(&mut ident);

    output.extend_from_slice(&[
        syntax::HighlightedSpan {
            text: ident_space,
            group: None,
        },
        syntax::HighlightedSpan {
            text: semicolon,
            group: Some(syntax::HighlightGroup::Terminator),
        },
    ]);

    Ok((s, output))
}

fn ty_alias(s: &str) -> ParseResult<'_> {
    let (s, keyword) = tag("type")(s)?;
    let (s, keyword_space) = take_whitespace1(s)?;

    let (s, name) = pascal_case(s)?;
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

    let (s, name) = snake_case(s)?;
    let (s, name_space) = take_whitespace0(s)?;

    let (s, open_paren) = tag(start_params)(s)?;
    let (s, open_paren_space) = take_whitespace0(s)?;

    let (s, mut params) = comma_separated(&function_def_param, end_params)(s)?;

    let (s, close_paren) = tag(end_params)(s)?;
    let (s, close_paren_space) = take_whitespace0(s)?;

    let (s, return_type) = opt(function_return_type)(s)?;
    let (s, return_type_space) = take_whitespace0(s)?;

    let semicolon = map(tag(";"), |s| {
        vec![syntax::HighlightedSpan {
            text: s,
            group: Some(syntax::HighlightGroup::Terminator),
        }]
    });

    // Function bodies can be either a block expression, or simply a semicolon (as in traits).
    let (s, mut body) = alt((block, semicolon))(s)?;

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
    let (s, name) = snake_case(s)?;
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

    let (s, name) = pascal_case(s)?;
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

    let (s, ty_annotation) = opt(|s| {
        let (s, colon) = tag(":")(s)?;
        let (s, colon_space) = take_whitespace0(s)?;

        let (s, mut ty) = ty(s)?;
        let (s, ty_space) = take_whitespace0(s)?;

        let mut output = vec![
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

        output.push(syntax::HighlightedSpan {
            text: ty_space,
            group: None,
        });

        Ok((s, output))
    })(s)?;

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

    if let Some(mut ty_annotation) = ty_annotation {
        output.append(&mut ty_annotation);
    }

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
    let (s, expr) = alt((function_call, variable, string, character, int))(s)?;

    let (s, postfixes) = many0(alt((method_call, field_access, try_)))(s)?;

    let mut output = expr;
    output.append(&mut postfixes.concat());

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

    let (s, field) = snake_case(s)?;

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

fn try_(s: &str) -> ParseResult<'_> {
    map(tag("?"), |s| {
        vec![syntax::HighlightedSpan {
            text: s,
            group: Some(syntax::HighlightGroup::OtherOper),
        }]
    })(s)
}

fn function_call(s: &str) -> ParseResult<'_> {
    let (s, path) = path(s)?;
    let (s, name) = snake_case(s)?;
    let (s, name_space) = take_whitespace0(s)?;

    let (s, open_paren) = tag("(")(s)?;
    let (s, open_paren_space) = take_whitespace0(s)?;

    // Function calls take in expressions.
    let (s, mut params) = comma_separated(&expr, ")")(s)?;
    let (s, params_space) = take_whitespace0(s)?;

    let (s, close_paren) = tag(")")(s)?;

    let mut output = path;

    output.extend_from_slice(&[
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
    ]);

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
    map(snake_case, |s| {
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

fn character(s: &str) -> ParseResult<'_> {
    let (s, start_quote) = tag("'")(s)?;
    let (s, contents) = take(1usize)(s)?;
    let (s, end_quote) = tag("'")(s)?;

    let output = vec![
        syntax::HighlightedSpan {
            text: start_quote,
            group: Some(syntax::HighlightGroup::CharacterDelimiter),
        },
        syntax::HighlightedSpan {
            text: contents,
            group: Some(syntax::HighlightGroup::Character),
        },
        syntax::HighlightedSpan {
            text: end_quote,
            group: Some(syntax::HighlightGroup::CharacterDelimiter),
        },
    ];

    Ok((s, output))
}

fn int(s: &str) -> ParseResult<'_> {
    let (s, number) = alt((binary, octal, hex, decimal))(s)?;
    let (s, suffix) = opt(int_ty)(s)?;

    let mut output = number;

    if let Some(suffix) = suffix {
        output.push(syntax::HighlightedSpan {
            text: suffix,
            group: Some(syntax::HighlightGroup::Number),
        });
    }

    Ok((s, output))
}

fn decimal(s: &str) -> ParseResult<'_> {
    map(digits(|c| c.is_ascii_digit()), |s| {
        vec![syntax::HighlightedSpan {
            text: s,
            group: Some(syntax::HighlightGroup::Number),
        }]
    })(s)
}

fn binary(s: &str) -> ParseResult<'_> {
    let (s, leader) = tag("0b")(s)?;
    let (s, underscore) = opt(tag("_"))(s)?;
    let (s, digits) = digits(|c| c == '0' || c == '1')(s)?;

    let mut output = vec![syntax::HighlightedSpan {
        text: leader,
        group: Some(syntax::HighlightGroup::Number),
    }];

    if let Some(underscore) = underscore {
        output.push(syntax::HighlightedSpan {
            text: underscore,
            group: Some(syntax::HighlightGroup::Number),
        });
    }

    output.push(syntax::HighlightedSpan {
        text: digits,
        group: Some(syntax::HighlightGroup::Number),
    });

    Ok((s, output))
}

fn octal(s: &str) -> ParseResult<'_> {
    let (s, leader) = tag("0o")(s)?;
    let (s, underscore) = opt(tag("_"))(s)?;
    let (s, digits) = digits(|c| c >= '0' && c <= '7')(s)?;

    let mut output = vec![syntax::HighlightedSpan {
        text: leader,
        group: Some(syntax::HighlightGroup::Number),
    }];

    if let Some(underscore) = underscore {
        output.push(syntax::HighlightedSpan {
            text: underscore,
            group: Some(syntax::HighlightGroup::Number),
        });
    }

    output.push(syntax::HighlightedSpan {
        text: digits,
        group: Some(syntax::HighlightGroup::Number),
    });

    Ok((s, output))
}

fn hex(s: &str) -> ParseResult<'_> {
    let (s, leader) = tag("0x")(s)?;
    let (s, underscore) = opt(tag("_"))(s)?;
    let (s, digits) =
        digits(|c| c.is_ascii_digit() || (c >= 'a' && c <= 'f') || (c >= 'A' && c <= 'F'))(s)?;

    let mut output = vec![syntax::HighlightedSpan {
        text: leader,
        group: Some(syntax::HighlightGroup::Number),
    }];

    if let Some(underscore) = underscore {
        output.push(syntax::HighlightedSpan {
            text: underscore,
            group: Some(syntax::HighlightGroup::Number),
        });
    }

    output.push(syntax::HighlightedSpan {
        text: digits,
        group: Some(syntax::HighlightGroup::Number),
    });

    Ok((s, output))
}

fn pattern(s: &str) -> ParseResult<'_> {
    map(snake_case, |s| {
        vec![syntax::HighlightedSpan {
            text: s,
            group: Some(syntax::HighlightGroup::VariableDef),
        }]
    })(s)
}

fn ty(s: &str) -> ParseResult<'_> {
    let (s, path) = path(s)?;
    let (s, mut name) = ty_name(s)?;

    let mut output = path;
    output.append(&mut name);

    Ok((s, output))
}

fn ty_name(s: &str) -> ParseResult<'_> {
    let user_ty = map(pascal_case, |s| {
        vec![syntax::HighlightedSpan {
            text: s,
            group: Some(syntax::HighlightGroup::TyUse),
        }]
    });

    alt((user_ty, primitive_ty))(s)
}

fn primitive_ty(s: &str) -> ParseResult<'_> {
    map(
        alt((int_ty, float_ty, tag("str"), tag("bool"), tag("!"))),
        |s| {
            vec![syntax::HighlightedSpan {
                text: s,
                group: Some(syntax::HighlightGroup::PrimitiveTy),
            }]
        },
    )(s)
}

fn path(s: &str) -> ParseResult<'_> {
    let (s, leading_colons) = opt(tag("::"))(s)?;

    let (s, modules) = many0(|s| {
        let (s, module_name) = module_name(s)?;
        let (s, double_colon) = tag("::")(s)?;

        let mut output = module_name;

        output.push(syntax::HighlightedSpan {
            text: double_colon,
            group: Some(syntax::HighlightGroup::Separator),
        });

        Ok((s, output))
    })(s)?;

    let mut output = if let Some(leading_colons) = leading_colons {
        vec![syntax::HighlightedSpan {
            text: leading_colons,
            group: Some(syntax::HighlightGroup::Separator),
        }]
    } else {
        vec![]
    };

    output.append(&mut modules.concat());

    Ok((s, output))
}

fn module_name(s: &str) -> ParseResult<'_> {
    map(snake_case, |s| {
        vec![syntax::HighlightedSpan {
            text: s,
            group: Some(syntax::HighlightGroup::ModuleUse),
        }]
    })(s)
}
