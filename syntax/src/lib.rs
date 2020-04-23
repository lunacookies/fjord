//! Types and traits for implementing syntax highlighting.

#![warn(missing_debug_implementations, missing_docs, rust_2018_idioms)]

/// This trait is to be implemented by any type that syntax highlights source code for a particular
/// language. This is done by taking in a string slice and outputting a vector of
/// [`Span`](struct.Span.html)s.
pub trait Highlight {
    /// Ensure that all input text is also contained in the `text` fields of the outputted `Span`s
    /// – in other words, this function must be lossless.
    fn highlight<'input>(&self, input: &'input str) -> Vec<Span<'input>>;
}

/// An individual fragment of highlighted text.
#[derive(Clone, Copy, Debug)]
pub struct Span<'text> {
    /// the text being highlighted
    pub text: &'text str,
    /// the highlight group it has been assigned
    pub group: HighlightGroup,
}

/// The set of possible syntactical forms text can be assigned.
///
/// As it is certain that more variants will be added in future, this enum has been marked as
/// non-exhaustive. It is recommended that the wildcard that this implies catches not only any
/// future new variants, but also the `Unhighlighted` variant – this way you conveniently specify
/// that highlight groups you haven’t defined styles for yet get the same (lack of) highlighting
/// that unhighlighted text gets.
#[non_exhaustive]
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, strum_macros::EnumIter)]
pub enum HighlightGroup {
    Unhighlighted,
    Keyword,
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

/// The styling applied to a given [`HighlightGroup`](enum.HighlightGroup.html).
#[derive(Clone, Copy, Debug)]
pub struct Style {
    /// its foreground color
    pub fg_color: Rgb,
    /// its (optional) background color
    pub bg_color: Option<Rgb>,
}

/// A trait for defining syntax highlighting themes.
pub trait Theme {
    /// Provides a mapping from `HighlightGroup`s to `Style`s. As `HighlightGroup`s contain a
    /// variant for unhighlighted text, this thereby defines the appearance of the whole text
    /// field.
    fn style(&self, group: HighlightGroup) -> Style;
}
