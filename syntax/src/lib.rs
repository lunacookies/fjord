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
///
/// As it is certain that more variants will be added in future, this enum has been marked as
/// non-exhaustive.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, strum_macros::EnumIter)]
pub enum HighlightGroup {
    /// a keyword, e.g. `if`
    Keyword,
    /// the name of a function
    Function,
    /// the name of a module
    Module,
    /// an operator that accesses the members of something, whether this is some kind of ‘object’
    /// or a module, e.g. ‘.’ and ‘::’ in Rust
    MemberOper,
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

/// An RGB color.
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
#[derive(Clone, Copy, Debug, Default)]
pub struct Style {
    /// its (optional) foreground color
    pub fg_color: Option<Rgb>,
    /// its (optional) background color
    pub bg_color: Option<Rgb>,
}

impl Style {
    /// Creates a new Style without a foreground or background colour.
    pub fn new() -> Self {
        Self {
            fg_color: None,
            bg_color: None,
        }
    }

    fn resolve(self, resolved: ResolvedStyle) -> ResolvedStyle {
        ResolvedStyle {
            fg_color: self.fg_color.unwrap_or(resolved.fg_color),
            bg_color: self.bg_color.unwrap_or(resolved.bg_color),
        }
    }
}

/// Identical to a [`Style`](struct.Style.html), except that it must have a background color. This
/// is outputted by (`render`)(fn.render.html), which resolves the background colour of every
/// [`Style`](struct.Style.html) it encounters.
#[derive(Clone, Copy, Debug)]
pub struct ResolvedStyle {
    /// its foreground color
    pub fg_color: Rgb,
    /// its background color
    pub bg_color: Rgb,
}

impl From<ResolvedStyle> for ansi_term::Style {
    fn from(style: ResolvedStyle) -> Self {
        let fg_color: ansi_term::Colour = style.fg_color.into();
        let bg_color: ansi_term::Colour = style.bg_color.into();

        fg_color.on(bg_color)
    }
}

/// A trait for defining syntax highlighting themes.
pub trait Theme {
    /// The style for unhighlighted text. To understand why this must be a fully resolved style,
    /// consider the following example:
    ///
    /// - `default_style` returns a [`Style`](struct.Style.html) which omits a foreground color -
    /// at some point a [highlighter](trait.Highlight.html) returns a
    /// [`HighlightedSpan`](struct.HighlightedSpan.html) without a highlight group
    /// - when [`render`](fn.render.html) is called, what is the foreground color of this
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
