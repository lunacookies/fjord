pub(crate) struct DarkPlus;

const DARK_BLUE: syntax::Rgb = syntax::rgb!(86, 156, 214);
const DARK_GREEN: syntax::Rgb = syntax::rgb!(107, 153, 85);
const DULL_GREEN_DARKER: syntax::Rgb = syntax::rgb!(181, 206, 168);
const FADED: syntax::Rgb = syntax::rgb!(178, 178, 178);
const GREEN: syntax::Rgb = syntax::rgb!(134, 198, 145);
const DULL_GREEN: syntax::Rgb = syntax::rgb!(184, 215, 163);
const LIGHT_BLUE: syntax::Rgb = syntax::rgb!(156, 220, 254);
const ORANGE: syntax::Rgb = syntax::rgb!(206, 144, 120);
const PURPLE: syntax::Rgb = syntax::rgb!(197, 134, 192);
const RED: syntax::Rgb = syntax::rgb!(244, 71, 71);
const TEAL: syntax::Rgb = syntax::rgb!(78, 201, 176);
const YELLOW: syntax::Rgb = syntax::rgb!(220, 220, 170);

impl syntax::Theme for DarkPlus {
    fn default_style(&self) -> syntax::ResolvedStyle {
        syntax::ResolvedStyle {
            fg_color: syntax::rgb!(212, 212, 212),
            bg_color: syntax::rgb!(30, 30, 30),
            is_bold: false,
            is_italic: false,
            is_underline: false,
        }
    }

    fn style(&self, group: syntax::HighlightGroup) -> syntax::Style {
        match group {
            syntax::HighlightGroup::CtrlFlowKeyword => syntax::Style {
                fg_color: Some(PURPLE),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            // Keywords and things that are often treated as such
            syntax::HighlightGroup::OtherKeyword
            | syntax::HighlightGroup::PrimitiveTy
            | syntax::HighlightGroup::Boolean => syntax::Style {
                fg_color: Some(DARK_BLUE),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            // Call-able things
            syntax::HighlightGroup::FunctionDef
            | syntax::HighlightGroup::FunctionCall
            | syntax::HighlightGroup::MacroDef
            | syntax::HighlightGroup::MacroUse => syntax::Style {
                fg_color: Some(YELLOW),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            syntax::HighlightGroup::TyDef | syntax::HighlightGroup::TyUse => syntax::Style {
                fg_color: Some(TEAL),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            syntax::HighlightGroup::InterfaceDef | syntax::HighlightGroup::InterfaceUse => {
                syntax::Style {
                    fg_color: Some(DULL_GREEN),
                    bg_color: None,
                    is_bold: false,
                    is_italic: false,
                    is_underline: false,
                }
            }

            syntax::HighlightGroup::VariableDef
            | syntax::HighlightGroup::VariableUse
            | syntax::HighlightGroup::MemberDef
            | syntax::HighlightGroup::MemberUse
            | syntax::HighlightGroup::ConstantDef
            | syntax::HighlightGroup::ConstantUse
            | syntax::HighlightGroup::FunctionParam => syntax::Style {
                fg_color: Some(LIGHT_BLUE),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            syntax::HighlightGroup::SpecialIdentDef | syntax::HighlightGroup::SpecialIdentUse => {
                syntax::Style {
                    // This colour is actually used for structs, but the distinction between
                    // structs and other types is only possible through semantic highlighting -- it
                    // is expected that all highlighters will either be simple lexers or parsers.
                    //
                    // Since ‘special identifiers’ are unique in the languages that they occur in
                    // (e.g.  lifetimes in Rust, symbols in Ruby), it makes sense to give them a
                    // special colour. This colour was left over, so I decided to use it.
                    fg_color: Some(GREEN),
                    bg_color: None,
                    is_bold: false,
                    is_italic: false,
                    is_underline: false,
                }
            }

            // Modules aren’t highlighted
            syntax::HighlightGroup::ModuleDef | syntax::HighlightGroup::ModuleUse => {
                syntax::Style {
                    fg_color: None,
                    bg_color: None,
                    is_bold: false,
                    is_italic: false,
                    is_underline: false,
                }
            }

            syntax::HighlightGroup::Number => syntax::Style {
                fg_color: Some(DULL_GREEN_DARKER),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            syntax::HighlightGroup::String
            | syntax::HighlightGroup::StringDelimiter
            | syntax::HighlightGroup::Character
            | syntax::HighlightGroup::CharacterDelimiter => syntax::Style {
                fg_color: Some(ORANGE),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            syntax::HighlightGroup::PreProc => syntax::Style {
                fg_color: Some(DARK_BLUE),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            syntax::HighlightGroup::Attribute => syntax::Style {
                fg_color: Some(FADED),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            syntax::HighlightGroup::Comment | syntax::HighlightGroup::DocComment => syntax::Style {
                fg_color: Some(DARK_GREEN),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            // Punctuation
            syntax::HighlightGroup::MemberOper
            | syntax::HighlightGroup::PointerOper
            | syntax::HighlightGroup::AssignOper
            | syntax::HighlightGroup::BinaryOper
            | syntax::HighlightGroup::OtherOper
            | syntax::HighlightGroup::Delimiter
            | syntax::HighlightGroup::Separator
            | syntax::HighlightGroup::Terminator => syntax::Style {
                fg_color: Some(FADED),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            syntax::HighlightGroup::Error => syntax::Style {
                fg_color: Some(RED),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: true,
            },
        }
    }
}
