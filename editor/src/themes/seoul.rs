pub(crate) struct Seoul;

const BLUE: syntax::Rgb = syntax::rgb!(152, 190, 222);
const BROWN: syntax::Rgb = syntax::rgb!(190, 152, 115);
const CREAM: syntax::Rgb = syntax::rgb!(223, 222, 189);
const CYAN: syntax::Rgb = syntax::rgb!(111, 188, 189);
const DARK_GREEN: syntax::Rgb = syntax::rgb!(113, 152, 114);
const GREEN: syntax::Rgb = syntax::rgb!(152, 188, 153);
const KHAKI: syntax::Rgb = syntax::rgb!(189, 187, 114);
const LEMON: syntax::Rgb = syntax::rgb!(222, 221, 153);
const LIGHT_BLUE: syntax::Rgb = syntax::rgb!(152, 188, 189);
const LIGHT_YELLOW: syntax::Rgb = syntax::rgb!(255, 222, 153);
const PURPLE: syntax::Rgb = syntax::rgb!(225, 120, 153);
const SALMON: syntax::Rgb = syntax::rgb!(255, 191, 189);
const VIOLET: syntax::Rgb = syntax::rgb!(153, 154, 189);
const YELLOW: syntax::Rgb = syntax::rgb!(223, 188, 114);

impl syntax::Theme for Seoul {
    fn default_style(&self) -> syntax::ResolvedStyle {
        syntax::ResolvedStyle {
            fg_color: syntax::rgb!(217, 217, 217),
            bg_color: syntax::rgb!(75, 75, 75),
            is_bold: false,
            is_italic: false,
            is_underline: false,
        }
    }

    fn style(&self, group: syntax::HighlightGroup) -> syntax::Style {
        match group {
            syntax::HighlightGroup::CtrlFlowKeyword => syntax::Style {
                fg_color: Some(BLUE),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            syntax::HighlightGroup::OtherKeyword => syntax::Style {
                fg_color: Some(GREEN),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            syntax::HighlightGroup::FunctionDef | syntax::HighlightGroup::FunctionCall => {
                syntax::Style {
                    fg_color: Some(CREAM),
                    bg_color: None,
                    is_bold: false,
                    is_italic: false,
                    is_underline: false,
                }
            }

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

            syntax::HighlightGroup::VariableDef
            | syntax::HighlightGroup::VariableUse
            | syntax::HighlightGroup::FunctionParam => syntax::Style {
                fg_color: None,
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            syntax::HighlightGroup::MemberDef | syntax::HighlightGroup::MemberUse => {
                syntax::Style {
                    fg_color: Some(SALMON),
                    bg_color: None,
                    is_bold: false,
                    is_italic: false,
                    is_underline: false,
                }
            }

            syntax::HighlightGroup::ConstantDef | syntax::HighlightGroup::ConstantUse => {
                syntax::Style {
                    fg_color: Some(SALMON),
                    bg_color: None,
                    is_bold: false,
                    is_italic: false,
                    is_underline: false,
                }
            }

            syntax::HighlightGroup::ModuleDef | syntax::HighlightGroup::ModuleUse => {
                syntax::Style {
                    fg_color: None,
                    bg_color: None,
                    is_bold: false,
                    is_italic: false,
                    is_underline: false,
                }
            }

            syntax::HighlightGroup::MacroDef
            | syntax::HighlightGroup::MacroUse
            | syntax::HighlightGroup::PreProc => syntax::Style {
                fg_color: Some(KHAKI),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            syntax::HighlightGroup::SpecialIdentDef | syntax::HighlightGroup::SpecialIdentUse => {
                syntax::Style {
                    fg_color: Some(CYAN),
                    bg_color: None,
                    is_bold: false,
                    is_italic: false,
                    is_underline: false,
                }
            }

            syntax::HighlightGroup::Number => syntax::Style {
                fg_color: Some(LIGHT_YELLOW),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            syntax::HighlightGroup::String | syntax::HighlightGroup::Character => syntax::Style {
                fg_color: Some(LIGHT_BLUE),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            syntax::HighlightGroup::StringDelimiter
            | syntax::HighlightGroup::CharacterDelimiter => syntax::Style {
                fg_color: Some(BROWN),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            syntax::HighlightGroup::Boolean => syntax::Style {
                fg_color: Some(VIOLET),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            syntax::HighlightGroup::Attribute => syntax::Style {
                fg_color: Some(GREEN),
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

            syntax::HighlightGroup::MemberOper
            | syntax::HighlightGroup::PointerOper
            | syntax::HighlightGroup::AssignOper
            | syntax::HighlightGroup::BinaryOper
            | syntax::HighlightGroup::OtherOper => syntax::Style {
                fg_color: Some(LEMON),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            syntax::HighlightGroup::Separator
            | syntax::HighlightGroup::Delimiter
            | syntax::HighlightGroup::Terminator => syntax::Style {
                fg_color: None,
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            syntax::HighlightGroup::Error => syntax::Style {
                fg_color: Some(PURPLE),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: true,
            },
        }
    }
}
