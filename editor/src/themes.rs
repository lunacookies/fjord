pub(crate) struct Gruvbox;

const GRAY: syntax::Rgb = syntax::rgb!(189, 174, 147);
const RED: syntax::Rgb = syntax::rgb!(251, 73, 52);
const GREEN: syntax::Rgb = syntax::rgb!(184, 187, 38);
const YELLOW: syntax::Rgb = syntax::rgb!(250, 189, 47);
const BLUE: syntax::Rgb = syntax::rgb!(131, 165, 152);
const PURPLE: syntax::Rgb = syntax::rgb!(211, 134, 155);
const AQUA: syntax::Rgb = syntax::rgb!(142, 192, 124);
const ORANGE: syntax::Rgb = syntax::rgb!(254, 128, 25);

impl syntax::Theme for Gruvbox {
    fn default_style(&self) -> syntax::ResolvedStyle {
        syntax::ResolvedStyle {
            fg_color: syntax::rgb!(235, 219, 178),
            bg_color: syntax::rgb!(29, 32, 33),
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
                    fg_color: Some(RED),
                    bg_color: None,
                    is_bold: true,
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
            | syntax::HighlightGroup::InterfaceDef
            | syntax::HighlightGroup::InterfaceUse
            | syntax::HighlightGroup::PrimitiveTy => syntax::Style {
                fg_color: Some(YELLOW),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            // Variables
            syntax::HighlightGroup::VariableDef
            | syntax::HighlightGroup::VariableUse
            | syntax::HighlightGroup::FunctionParam => syntax::Style {
                fg_color: Some(BLUE),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            // Constants
            syntax::HighlightGroup::ConstantDef
            | syntax::HighlightGroup::ConstantUse
            | syntax::HighlightGroup::Number
            | syntax::HighlightGroup::Boolean => syntax::Style {
                fg_color: Some(PURPLE),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            // Modules
            syntax::HighlightGroup::ModuleDef | syntax::HighlightGroup::ModuleUse => {
                syntax::Style {
                    fg_color: Some(BLUE),
                    bg_color: None,
                    is_bold: false,
                    is_italic: false,
                    is_underline: false,
                }
            }

            // Preprocessor-related
            syntax::HighlightGroup::MacroDef
            | syntax::HighlightGroup::MacroUse
            | syntax::HighlightGroup::PreProc
            | syntax::HighlightGroup::Attribute => syntax::Style {
                fg_color: Some(AQUA),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            // String and character literals
            syntax::HighlightGroup::String
            | syntax::HighlightGroup::StringDelimiter
            | syntax::HighlightGroup::Character
            | syntax::HighlightGroup::CharacterDelimiter => syntax::Style {
                fg_color: Some(GREEN),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            // Special identifiers
            syntax::HighlightGroup::SpecialIdentDef
            | syntax::HighlightGroup::SpecialIdentUse
            | syntax::HighlightGroup::SpecialIdentDefSigil
            | syntax::HighlightGroup::SpecialIdentUseSigil => syntax::Style {
                fg_color: Some(ORANGE),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            // Comments
            syntax::HighlightGroup::Comment | syntax::HighlightGroup::DocComment => syntax::Style {
                fg_color: Some(GRAY),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            // Punctuation
            syntax::HighlightGroup::MemberOper
            | syntax::HighlightGroup::PointerOper
            | syntax::HighlightGroup::AssignOper
            | syntax::HighlightGroup::Delimiter
            | syntax::HighlightGroup::Separator
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
                is_underline: true,
            },
        }
    }
}
