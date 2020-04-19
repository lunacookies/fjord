use nom::bytes::complete::take_while1;

/// An identifier (e.g. variable name, function name). Currently, all `IdentName`s have to follow
/// camelCase. They can be matched by the regex `[a-z][a-zA-Z0-9]*`.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct IdentName(String);

impl IdentName {
    pub(crate) fn new(s: &str) -> nom::IResult<&str, Self> {
        let _ = take_while1(|c: char| c.is_ascii_lowercase())(s)?;
        let (s, name) = take_while1(|c: char| c.is_ascii_alphanumeric())(s)?;

        // Identifier names cannot contain keywords.
        if name.contains("if")
            || name.contains("then")
            || name.contains("else")
            || name.contains("true")
            || name.contains("false")
        {
            return Err(nom::Err::Error((s, nom::error::ErrorKind::Not)));
        }

        Ok((s, Self(name.into())))
    }

    /// Creates a new `IdentName`, panicking if the input does not perfectly parse.
    pub fn new_panicking(s: &str) -> Self {
        let (s, ident_name) = Self::new(s).unwrap();
        assert!(s == "");

        ident_name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn camel_case() {
        assert_eq!(
            IdentName::new("camelCase"),
            Ok(("", IdentName("camelCase".into())))
        )
    }

    #[test]
    fn pascal_case() {
        assert!(IdentName::new("PascalCase").is_err())
    }

    #[test]
    fn shouty_snake_case() {
        assert!(IdentName::new("SHOUTY_SNAKE_CASE").is_err())
    }

    // These two tests assert not that the cases fail, but that they don’t use up all of their
    // input.

    #[test]
    fn snake_case() {
        assert_ne!(IdentName::new("snake_case").unwrap().0, "");
    }

    #[test]
    fn kebab_case() {
        assert_ne!(IdentName::new("kebab-case").unwrap().0, "");
    }
}
