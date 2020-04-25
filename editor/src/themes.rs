pub(crate) struct Gruvbox;

const GRAY: syntax::Rgb = syntax::rgb!(189, 174, 147);
const RED: syntax::Rgb = syntax::rgb!(251, 73, 52);
const GREEN: syntax::Rgb = syntax::rgb!(184, 187, 38);

#[allow(dead_code)]
const YELLOW: syntax::Rgb = syntax::rgb!(250, 189, 47);

const BLUE: syntax::Rgb = syntax::rgb!(131, 165, 152);

#[allow(dead_code)]
const PURPLE: syntax::Rgb = syntax::rgb!(211, 134, 155);

#[allow(dead_code)]
const AQUA: syntax::Rgb = syntax::rgb!(142, 192, 124);

#[allow(dead_code)]
const ORANGE: syntax::Rgb = syntax::rgb!(254, 128, 25);

impl syntax::Theme for Gruvbox {
    fn default_style(&self) -> syntax::ResolvedStyle {
        syntax::ResolvedStyle {
            fg_color: syntax::Rgb {
                r: 235,
                g: 219,
                b: 178,
            },
            bg_color: syntax::Rgb {
                r: 29,
                g: 32,
                b: 33,
            },
            is_bold: false,
            is_italic: false,
            is_underline: false,
        }
    }

    fn style(&self, group: syntax::HighlightGroup) -> syntax::Style {
        match group {
            syntax::HighlightGroup::Keyword => syntax::Style {
                fg_color: Some(RED),
                bg_color: None,
                is_bold: true,
                is_italic: false,
                is_underline: false,
            },
            syntax::HighlightGroup::Function => syntax::Style {
                fg_color: Some(GREEN),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },
            syntax::HighlightGroup::Module => syntax::Style {
                fg_color: Some(BLUE),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },
            syntax::HighlightGroup::MemberOper => syntax::Style {
                fg_color: Some(GRAY),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },
            },
            _ => syntax::Style::default(),
        }
    }
}
