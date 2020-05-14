pub(crate) struct Dracula;

const FADED: syntax::Rgb = syntax::rgb!(98, 114, 164);
const RED: syntax::Rgb = syntax::rgb!(255, 85, 85);
const ORANGE: syntax::Rgb = syntax::rgb!(255, 184, 108);
const YELLOW: syntax::Rgb = syntax::rgb!(241, 250, 140);
const GREEN: syntax::Rgb = syntax::rgb!(80, 250, 123);
const PURPLE: syntax::Rgb = syntax::rgb!(189, 147, 249);
const CYAN: syntax::Rgb = syntax::rgb!(139, 233, 253);
const PINK: syntax::Rgb = syntax::rgb!(255, 121, 198);

impl syntax::Theme for Dracula {
    fn default_style(&self) -> syntax::ResolvedStyle {
        syntax::ResolvedStyle {
            fg_color: syntax::rgb!(248, 248, 242),
            bg_color: syntax::rgb!(40, 42, 54),
            is_bold: false,
            is_italic: false,
            is_underline: false,
        }
    }

    fn style(&self, group: syntax::HighlightGroup) -> syntax::Style {
        match group {
            // Keywords
            syntax::HighlightGroup::CtrlFlowKeyword | syntax::HighlightGroup::OtherKeyword => {
                syntax::Style {
                    fg_color: Some(PINK),
                    bg_color: None,
                    is_bold: false,
                    is_italic: false,
                    is_underline: false,
                }
            }

            // Functions
            syntax::HighlightGroup::FunctionDef | syntax::HighlightGroup::FunctionCall => {
                syntax::Style {
                    fg_color: Some(GREEN),
                    bg_color: None,
                    is_bold: false,
                    is_italic: false,
                    is_underline: false,
                }
            }

            // Types
            syntax::HighlightGroup::TyDef
            | syntax::HighlightGroup::TyUse
            | syntax::HighlightGroup::PrimitiveTy => syntax::Style {
                fg_color: Some(CYAN),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            // Interfaces
            syntax::HighlightGroup::InterfaceDef | syntax::HighlightGroup::InterfaceUse => {
                syntax::Style {
                    fg_color: Some(CYAN),
                    bg_color: None,
                    is_bold: false,
                    is_italic: true,
                    is_underline: false,
                }
            }

            // Variables
            syntax::HighlightGroup::VariableDef
            | syntax::HighlightGroup::VariableUse
            | syntax::HighlightGroup::MemberDef
            | syntax::HighlightGroup::MemberUse => syntax::Style {
                fg_color: None,
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            // Function parameters
            syntax::HighlightGroup::FunctionParam => syntax::Style {
                fg_color: Some(ORANGE),
                bg_color: None,
                is_bold: false,
                is_italic: true,
                is_underline: false,
            },

            // Constants
            syntax::HighlightGroup::ConstantDef
            | syntax::HighlightGroup::ConstantUse
            | syntax::HighlightGroup::Number
            | syntax::HighlightGroup::Boolean
            | syntax::HighlightGroup::Character
            | syntax::HighlightGroup::CharacterDelimiter => syntax::Style {
                fg_color: Some(PURPLE),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            // Modules
            syntax::HighlightGroup::ModuleDef | syntax::HighlightGroup::ModuleUse => {
                syntax::Style {
                    fg_color: None,
                    bg_color: None,
                    is_bold: false,
                    is_italic: false,
                    is_underline: false,
                }
            }

            // Macros and other preprocessor-related highlight groups
            syntax::HighlightGroup::MacroDef
            | syntax::HighlightGroup::MacroUse
            | syntax::HighlightGroup::PreProc => syntax::Style {
                fg_color: Some(CYAN),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            // Attributes
            syntax::HighlightGroup::Attribute => syntax::Style {
                fg_color: Some(GREEN),
                bg_color: None,
                is_bold: false,
                is_italic: true,
                is_underline: false,
            },

            // Strings
            syntax::HighlightGroup::String | syntax::HighlightGroup::StringDelimiter => {
                syntax::Style {
                    fg_color: Some(YELLOW),
                    bg_color: None,
                    is_bold: false,
                    is_italic: false,
                    is_underline: false,
                }
            }

            // Special identifiers
            syntax::HighlightGroup::SpecialIdentDef | syntax::HighlightGroup::SpecialIdentUse => {
                syntax::Style {
                    fg_color: Some(GREEN),
                    bg_color: None,
                    is_bold: false,
                    is_italic: false,
                    is_underline: false,
                }
            }

            // Comments
            syntax::HighlightGroup::Comment | syntax::HighlightGroup::DocComment => syntax::Style {
                fg_color: Some(FADED),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            // Some punctuation gets a colour
            syntax::HighlightGroup::PointerOper
            | syntax::HighlightGroup::AssignOper
            | syntax::HighlightGroup::BinaryOper
            | syntax::HighlightGroup::OtherOper
            | syntax::HighlightGroup::Separator => syntax::Style {
                fg_color: Some(PINK),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            // Other punctuation doesn’t
            syntax::HighlightGroup::MemberOper
            | syntax::HighlightGroup::Delimiter
            | syntax::HighlightGroup::Terminator => syntax::Style {
                fg_color: None,
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            // Errors
            syntax::HighlightGroup::Error => syntax::Style {
                fg_color: Some(RED),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },
        }
    }
}
