use {std::fmt, thiserror::Error};

#[derive(Debug, PartialEq)]
pub enum LexItem<'a> {
    CommandName(&'a str),
    CommandArg(&'a str),
}

impl AsRef<str> for LexItem<'_> {
    fn as_ref(&self) -> &str {
        match self {
            LexItem::CommandName(name) => name,
            LexItem::CommandArg(arg) => arg,
        }
    }
}

impl fmt::Display for LexItem<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

pub struct LexItemsIter<'a> {
    idx: usize,
    source: &'a str,
}

#[derive(Error, Debug, PartialEq)]
pub enum LexItemParseError {
    #[error("unmatched quote")]
    UnmatchedQuote,
}

impl<'a> Iterator for LexItemsIter<'a> {
    type Item = Result<LexItem<'a>, LexItemParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.source = self.source.trim();

        if self.source.is_empty() {
            return None;
        }

        // Try to find quoted region from start of source string, but if we don’t find one return
        // the first space-separated word.
        //
        // NOTE: We separate out end_idx because quotes require the special case of the end of the
        // lex item being different from the part of the source string that needs to be removed.
        // (the end quote)
        let (lex_item_contents, end_idx) = if self.source.chars().nth(0) == Some('"') {
            let closing_quote = match self.source[1..].find('"') {
                Some(idx) => idx,
                None => return Some(Err(LexItemParseError::UnmatchedQuote)),
            };

            (&self.source[1..closing_quote + 1], closing_quote + 2)
        } else {
            let word_end = self.source.find(char::is_whitespace);

            let (start, end) = match word_end {
                Some(word_end) => (0, word_end),
                None => (0, self.source.len()),
            };

            (&self.source[start..end], end)
        };

        self.source = &self.source[end_idx..];

        let lex_item = if self.idx == 0 {
            LexItem::CommandName(lex_item_contents)
        } else {
            LexItem::CommandArg(lex_item_contents)
        };

        self.idx += 1;

        Some(Ok(lex_item))
    }
}

impl<'a> LexItemsIter<'a> {
    pub fn new(s: &'a str) -> Self {
        Self { idx: 0, source: s }
    }
}

#[cfg(test)]
mod tests {
    use {super::*, proptest::prelude::*};

    #[test]
    fn basic() {
        assert_eq!(
            vec![
                Ok(LexItem::CommandName("echo")),
                Ok(LexItem::CommandArg("Quoted together")),
                Ok(LexItem::CommandArg("or")),
                Ok(LexItem::CommandArg("separate"))
            ],
            LexItemsIter::new("echo \"Quoted together\" or separate ").collect::<Vec<_>>()
        );
    }

    proptest! {
        #[test]
        fn does_not_crash(s in r#"[\w\s]*"#) {
            let _ = LexItemsIter::new(&s).collect::<Vec<_>>();
        }

        #[test]
        fn isolate_quoted(
            //                   ╭ No control chars
            //                   │    ╭ No quotes
            command_name in r#"[^\pC\s"]+"#, // No whitespace in command name and free arg because
            free_arg     in r#"[^\pC\s"]+"#, // whitespace can separate these.
            quoted_arg1  in r#"[^\pC"]+"#,   // Whitespace, however, is allowed for quoted args because
            quoted_arg2  in r#"[^\pC"]+"#,   // the quotes stop separation of the args.
        ) {
            let input = format!("{} \"{} {}\" {} ", command_name, quoted_arg1, quoted_arg2, free_arg);
            let arg1and2 = format!("{} {}", quoted_arg1, quoted_arg2);

            let lex_items = LexItemsIter::new(&input).collect::<Vec<_>>();

            prop_assert_eq!(
                vec![
                    Ok(LexItem::CommandName(&command_name)),
                    Ok(LexItem::CommandArg(&arg1and2)),
                    Ok(LexItem::CommandArg(&free_arg))
                ],
                lex_items
            );
        }
    }
}
