pub(crate) struct Monokai;

const FADED: syntax::Rgb = syntax::rgb!(117, 113, 94);

const CYAN: syntax::Rgb = syntax::rgb!(102, 217, 239);
const GREEN: syntax::Rgb = syntax::rgb!(166, 226, 46);
const ORANGE: syntax::Rgb = syntax::rgb!(253, 151, 31);
const PINK: syntax::Rgb = syntax::rgb!(249, 38, 114);
const PURPLE: syntax::Rgb = syntax::rgb!(174, 129, 255);
const YELLOW: syntax::Rgb = syntax::rgb!(230, 219, 116);

impl syntax::Theme for Monokai {
    fn default_style(&self) -> syntax::ResolvedStyle {
        syntax::ResolvedStyle {
            fg_color: syntax::rgb!(248, 248, 242),
            bg_color: syntax::rgb!(39, 40, 34),
            is_bold: false,
            is_italic: false,
            is_underline: false,
        }
    }

    fn style(&self, group: syntax::HighlightGroup) -> syntax::Style {
        match group {
            // Control flow and operators
            syntax::HighlightGroup::CtrlFlowKeyword
            | syntax::HighlightGroup::PointerOper
            | syntax::HighlightGroup::AssignOper => syntax::Style {
                fg_color: Some(PINK),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            syntax::HighlightGroup::OtherKeyword => syntax::Style {
                fg_color: Some(CYAN),
                bg_color: None,
                is_bold: false,
                is_italic: true,
                is_underline: false,
            },

            syntax::HighlightGroup::FunctionDef
            | syntax::HighlightGroup::TyDef
            | syntax::HighlightGroup::InterfaceDef
            | syntax::HighlightGroup::MacroDef => syntax::Style {
                fg_color: Some(GREEN),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            syntax::HighlightGroup::FunctionCall
            | syntax::HighlightGroup::MacroUse
            | syntax::HighlightGroup::PreProc => syntax::Style {
                fg_color: Some(CYAN),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            syntax::HighlightGroup::TyUse
            | syntax::HighlightGroup::InterfaceUse
            | syntax::HighlightGroup::PrimitiveTy => syntax::Style {
                fg_color: Some(CYAN),
                bg_color: None,
                is_bold: false,
                is_italic: true,
                is_underline: false,
            },

            // Variables, members and modules don’t get any highlighting
            syntax::HighlightGroup::VariableDef
            | syntax::HighlightGroup::VariableUse
            | syntax::HighlightGroup::MemberDef
            | syntax::HighlightGroup::MemberUse
            | syntax::HighlightGroup::ModuleDef
            | syntax::HighlightGroup::ModuleUse => syntax::Style {
                fg_color: None,
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            syntax::HighlightGroup::SpecialIdentDef
            | syntax::HighlightGroup::SpecialIdentUse
            | syntax::HighlightGroup::SpecialIdentDefSigil
            | syntax::HighlightGroup::SpecialIdentUseSigil => syntax::Style {
                fg_color: Some(PURPLE),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            syntax::HighlightGroup::FunctionParam => syntax::Style {
                fg_color: Some(ORANGE),
                bg_color: None,
                is_bold: false,
                is_italic: true,
                is_underline: false,
            },

            // Constants and literals (apart from strings)
            syntax::HighlightGroup::ConstantDef
            | syntax::HighlightGroup::ConstantUse
            | syntax::HighlightGroup::Number
            | syntax::HighlightGroup::Character
            | syntax::HighlightGroup::CharacterDelimiter
            | syntax::HighlightGroup::Boolean => syntax::Style {
                fg_color: Some(PURPLE),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            syntax::HighlightGroup::String | syntax::HighlightGroup::StringDelimiter => {
                syntax::Style {
                    fg_color: Some(YELLOW),
                    bg_color: None,
                    is_bold: false,
                    is_italic: false,
                    is_underline: false,
                }
            }

            syntax::HighlightGroup::Comment | syntax::HighlightGroup::DocComment => syntax::Style {
                fg_color: Some(FADED),
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

            syntax::HighlightGroup::Error => syntax::Style {
                fg_color: Some(PINK),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: true,
            },

            // Miscellaneous punctuation that doesn’t get highlighted
            syntax::HighlightGroup::MemberOper
            | syntax::HighlightGroup::Delimiter
            | syntax::HighlightGroup::Separator
            | syntax::HighlightGroup::Terminator => syntax::Style {
                fg_color: None,
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },
        }
    }
}
