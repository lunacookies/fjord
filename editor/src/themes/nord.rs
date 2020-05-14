// Some of the colours in the Nord palette aren’t used yet, but might be in the future.
#![allow(unused)]

pub(crate) struct Nord;

// Polar Night
const NORD0: syntax::Rgb = syntax::rgb!(46, 52, 64);
const NORD1: syntax::Rgb = syntax::rgb!(59, 66, 82);
const NORD2: syntax::Rgb = syntax::rgb!(67, 76, 94);
const NORD3: syntax::Rgb = syntax::rgb!(76, 86, 106);

// Snow Storm
const NORD4: syntax::Rgb = syntax::rgb!(216, 222, 233);
const NORD5: syntax::Rgb = syntax::rgb!(229, 233, 240);
const NORD6: syntax::Rgb = syntax::rgb!(236, 239, 244);

// Frost
const NORD7: syntax::Rgb = syntax::rgb!(143, 188, 187);
const NORD8: syntax::Rgb = syntax::rgb!(136, 192, 208);
const NORD9: syntax::Rgb = syntax::rgb!(129, 161, 193);
const NORD10: syntax::Rgb = syntax::rgb!(94, 129, 172);

// Aurora
const NORD11: syntax::Rgb = syntax::rgb!(191, 97, 106);
const NORD12: syntax::Rgb = syntax::rgb!(208, 135, 112);
const NORD13: syntax::Rgb = syntax::rgb!(235, 203, 139);
const NORD14: syntax::Rgb = syntax::rgb!(163, 190, 140);
const NORD15: syntax::Rgb = syntax::rgb!(180, 142, 173);

impl syntax::Theme for Nord {
    fn default_style(&self) -> syntax::ResolvedStyle {
        syntax::ResolvedStyle {
            fg_color: NORD6,
            bg_color: NORD0,
            is_bold: false,
            is_italic: false,
            is_underline: false,
        }
    }

    fn style(&self, group: syntax::HighlightGroup) -> syntax::Style {
        match group {
            syntax::HighlightGroup::CtrlFlowKeyword | syntax::HighlightGroup::OtherKeyword => {
                syntax::Style {
                    fg_color: Some(NORD9),
                    bg_color: None,
                    is_bold: false,
                    is_italic: false,
                    is_underline: false,
                }
            }

            syntax::HighlightGroup::FunctionDef | syntax::HighlightGroup::FunctionCall => {
                syntax::Style {
                    fg_color: Some(NORD8),
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
                fg_color: Some(NORD7),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            syntax::HighlightGroup::VariableDef
            | syntax::HighlightGroup::VariableUse
            | syntax::HighlightGroup::MemberDef
            | syntax::HighlightGroup::MemberUse
            | syntax::HighlightGroup::ConstantDef
            | syntax::HighlightGroup::ConstantUse
            | syntax::HighlightGroup::FunctionParam => syntax::Style {
                fg_color: Some(NORD4),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

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
                fg_color: Some(NORD10),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            // Unclear what highlighting this should get, as it is not specified by the Nord
            // Specification.
            syntax::HighlightGroup::SpecialIdentDef | syntax::HighlightGroup::SpecialIdentUse => {
                syntax::Style {
                    fg_color: Some(NORD7),
                    bg_color: None,
                    is_bold: false,
                    is_italic: false,
                    is_underline: false,
                }
            }

            syntax::HighlightGroup::Number => syntax::Style {
                fg_color: Some(NORD15),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            syntax::HighlightGroup::String | syntax::HighlightGroup::StringDelimiter => {
                syntax::Style {
                    fg_color: Some(NORD14),
                    bg_color: None,
                    is_bold: false,
                    is_italic: false,
                    is_underline: false,
                }
            }

            syntax::HighlightGroup::Character | syntax::HighlightGroup::CharacterDelimiter => {
                syntax::Style {
                    fg_color: Some(NORD13),
                    bg_color: None,
                    is_bold: false,
                    is_italic: false,
                    is_underline: false,
                }
            }

            syntax::HighlightGroup::Boolean => syntax::Style {
                fg_color: Some(NORD9),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            syntax::HighlightGroup::Attribute => syntax::Style {
                fg_color: Some(NORD12),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            syntax::HighlightGroup::Comment | syntax::HighlightGroup::DocComment => syntax::Style {
                fg_color: Some(NORD3),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            syntax::HighlightGroup::MemberOper
            | syntax::HighlightGroup::PointerOper
            | syntax::HighlightGroup::AssignOper
            | syntax::HighlightGroup::BinaryOper
            | syntax::HighlightGroup::OtherOper
            | syntax::HighlightGroup::Separator
            | syntax::HighlightGroup::Terminator => syntax::Style {
                fg_color: Some(NORD9),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            syntax::HighlightGroup::Delimiter => syntax::Style {
                fg_color: None,
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },

            syntax::HighlightGroup::Error => syntax::Style {
                fg_color: Some(NORD11),
                bg_color: None,
                is_bold: false,
                is_italic: false,
                is_underline: false,
            },
        }
    }
}
