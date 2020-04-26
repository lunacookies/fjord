//! Types and traits for implementing syntax highlighting.

#![warn(missing_debug_implementations, missing_docs, rust_2018_idioms)]

/// This trait is to be implemented by any type that syntax highlights source code for a particular
/// language. This is done by taking in a string slice and outputting a vector of
/// [`HighlightedSpan`](struct.HighlightedSpan.html)s.
pub trait Highlight {
    /// Ensure that all input text is also contained in the `text` fields of the outputted
    /// `HighlightedSpan`s – in other words, this function must be lossless.
    fn highlight<'input>(&self, input: &'input str) -> Vec<HighlightedSpan<'input>>;
}

/// An individual fragment of possibly highlighted text.
#[derive(Clone, Copy, Debug)]
pub struct HighlightedSpan<'text> {
    /// the text being highlighted
    pub text: &'text str,
    /// the highlight group it may have been assigned
    pub group: Option<HighlightGroup>,
}

/// The set of possible syntactical forms text can be assigned.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, strum_macros::EnumIter)]
pub enum HighlightGroup {
    /// a keyword that controls the flow of execution within code, e.g. `if` or `for`
    CtrlFlowKeyword,
    /// any other kind of keyword
    OtherKeyword,
    /// the name of a function when defined
    FunctionDef,
    /// the name of a function when called
    FunctionCall,
    /// the name of a type when defined
    TyDef,
    /// the name of a type when used
    TyUse,
    /// a ‘primitive’ baked into the language (e.g. `int` is a primitive type in C++, but
    /// `std::vector` isn’t)
    PrimitiveTy,
    /// the name of a variable when defined
    VariableDef,
    /// the name of a variable when used
    VariableUse,
    /// the name of a constant when defined
    ConstantDef,
    /// the name of a constant when used
    ConstantUse,
    /// the name of a module when defined
    ModuleDef,
    /// the name of a module when used
    ModuleUse,
    /// the name of a macro when defined
    MacroDef,
    /// the name of a macro when used
    MacroUse,
    /// the name of a special identifier (e.g. a symbol in Ruby or a lifetime in Rust) when defined
    SpecialIdentDef,
    /// the name of a special identifier when used
    SpecialIdentUse,
    /// a sigil used to indicate a special identifier
    SpecialIdentSigil,
    /// the name of a function parameter
    FunctionParam,
    /// a number literal (whether integer or floating-point)
    Number,
    /// a string literal
    String,
    /// the delimiters around a string literal (`"` in most languages)
    StringDelimiter,
    /// a character literal
    Character,
    /// a boolean literal
    Boolean,
    /// a pre-processor invocation that is not a macro itself (e.g. `#if` and `#define` in C)
    PreProc,
    /// the name of something that is an attribute of another thing (e.g. the word `derive` in
    /// `#[derive(Debug)]` in Rust, or a decorator in Python)
    Attribute,
    /// a comment
    Comment,
    /// a documentation comment
    DocComment,
    /// an operator that accesses the members of something, regardless of whether this is some kind
    /// of ‘object’ or a module, e.g. `.` and `::` in Rust
    MemberOper,
    /// an operator relating to pointers (e.g. `*` and `&` in C, those as well as `&mut` in Rust)
    PointerOper,
    /// an operator that assigns a value to a binding of some sort (`=` in most languages)
    AssignOper,
    /// a delimiter (e.g. `(`)
    Delimiter,
    /// a separator of something (e.g. `,` or `->`)
    Separator,
    /// a terminator of something (e.g. `;`)
    Terminator,
    /// an error
    Error,
}

/// An individual fragment of styled text.
#[derive(Clone, Copy, Debug)]
pub struct StyledSpan<'text> {
    /// the text being styled
    pub text: &'text str,
    /// the style it has been given
    pub style: ResolvedStyle,
}

impl<'text> StyledSpan<'text> {
    /// Splits the `StyledSpan` into a vector of `StyledSpan`s along the lines contained in the
    /// original `StyledSpan`. The outputs inherit the `style` of the original.
    pub fn split_lines(self) -> Vec<Self> {
        self.text
            .lines()
            .map(|text| Self {
                text,
                style: self.style,
            })
            .collect()
    }
}

/// An RGB colour.
#[derive(Clone, Copy, Debug)]
pub struct Rgb {
    /// red
    pub r: u8,
    /// green
    pub g: u8,
    /// blue
    pub b: u8,
}

impl From<Rgb> for ansi_term::Colour {
    fn from(rgb: Rgb) -> Self {
        Self::RGB(rgb.r, rgb.g, rgb.b)
    }
}

/// Allows easy creation of a [`Rgb`](struct.Rgb.html).
#[macro_export]
macro_rules! rgb {
    ($r:literal, $g:literal, $b:literal) => {
        $crate::Rgb {
            r: $r,
            g: $g,
            b: $b,
        }
    };
}

/// The styling applied to a given [`HighlightGroup`](enum.HighlightGroup.html).
///
/// When a field is given a `None` value, then that field’s value defaults to that of the theme’s
/// default style. It was decided that only colours are to be optional, because it is exceedingly
/// rare that an entire theme wishes to be bold, italic or underlined.
#[derive(Clone, Copy, Debug, Default)]
pub struct Style {
    /// its foreground colour
    pub fg_color: Option<Rgb>,
    /// its background colour
    pub bg_color: Option<Rgb>,
    /// whether to bolden
    pub is_bold: bool,
    /// whether to italicise
    pub is_italic: bool,
    /// whether to underline
    pub is_underline: bool,
}

impl Style {
    /// Creates a new Style with all colour fields set to `None` and all boolean fields set to
    /// false, thereby creating a style whose value is identical to that of the theme’s default
    /// style (assuming that the theme’s default style also uses false for all boolean options).
    pub fn new() -> Self {
        Self {
            fg_color: None,
            bg_color: None,
            is_bold: false,
            is_italic: false,
            is_underline: false,
        }
    }

    fn resolve(self, resolved: ResolvedStyle) -> ResolvedStyle {
        ResolvedStyle {
            fg_color: self.fg_color.unwrap_or(resolved.fg_color),
            bg_color: self.bg_color.unwrap_or(resolved.bg_color),
            is_bold: self.is_bold,
            is_italic: self.is_italic,
            is_underline: self.is_underline,
        }
    }
}

/// Identical to a [`Style`](struct.Style.html), except that all its fields are mandatory.
#[derive(Clone, Copy, Debug)]
pub struct ResolvedStyle {
    /// its foreground colour
    pub fg_color: Rgb,
    /// its background colour
    pub bg_color: Rgb,
    /// whether to bolden
    pub is_bold: bool,
    /// whether to italicise
    pub is_italic: bool,
    /// whether to underline
    pub is_underline: bool,
}

impl From<ResolvedStyle> for ansi_term::Style {
    fn from(style: ResolvedStyle) -> Self {
        Self {
            foreground: Some(style.fg_color.into()),
            background: Some(style.bg_color.into()),
            is_bold: style.is_bold,
            is_italic: style.is_italic,
            is_underline: style.is_underline,

            // These fields aren’t useful in the context of syntax highlighting, with the exception
            // of ‘is_dimmed’. The reason why ‘is_dimmed’ cannot be used by theme authors is that
            // its appearance depends on what colour the terminal picks, which can vary. This also
            // ensure consistency, thereby minimising support requests on themes (‘Why does it look
            // different to the screenshot?’).
            is_dimmed: false,
            is_blink: false,
            is_reverse: false,
            is_hidden: false,
            is_strikethrough: false,
        }
    }
}

/// A trait for defining syntax highlighting themes.
pub trait Theme {
    /// The style for unhighlighted text. To understand why this must be a fully resolved style,
    /// consider the following example:
    ///
    /// - `default_style` returns a [`Style`](struct.Style.html) which omits a foreground colour -
    /// at some point a [highlighter](trait.Highlight.html) returns a
    /// [`HighlightedSpan`](struct.HighlightedSpan.html) without a highlight group
    /// - when [`render`](fn.render.html) is called, what is the foreground colour of this
    ///   unhighlighted HighlightedSpan?
    ///
    /// To prevent situations like this, `default_style` acts as a fallback for all cases by
    /// forcing the implementor to define all of the style’s fields.
    fn default_style(&self) -> ResolvedStyle;

    /// Provides a mapping from `HighlightGroup`s to `Style`s. As `HighlightGroup`s contain a
    /// variant for unhighlighted text, this thereby defines the appearance of the whole text
    /// field.
    fn style(&self, group: HighlightGroup) -> Style;
}

/// A convenience function that renders a given input text using a given highlighter and theme,
/// returning a vector of `StyledSpan`s.
pub fn render<'input, H, T>(input: &'input str, highlighter: H, theme: T) -> Vec<StyledSpan<'input>>
where
    H: Highlight,
    T: Theme,
{
    use {std::collections::HashMap, strum::IntoEnumIterator};

    // The key is the highlight group, the value is the style the theme gives to this group.
    let styles: HashMap<_, _> = HighlightGroup::iter()
        .map(|group| (group, theme.style(group)))
        .collect();

    highlighter
        .highlight(input)
        .into_iter()
        .map(|span| {
            // If the span has a group assigned to it, then we use the resolved version of its
            // style. If the span does not have a highlight group, however, then we just use the
            // theme’s default style.
            let resolved_style = if let Some(group) = span.group {
                styles[&group].resolve(theme.default_style())
            } else {
                theme.default_style()
            };

            StyledSpan {
                text: span.text,
                style: resolved_style,
            }
        })
        .collect()
}
