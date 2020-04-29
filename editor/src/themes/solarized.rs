pub(crate) struct Monokai;

const BASE03: syntax::Rgb = syntax::rgb!(0, 43, 54);
const BASE01: syntax::Rgb = syntax::rgb!(88, 110, 117);
const BASE00: syntax::Rgb = syntax::rgb!(101, 123, 131);
const BASE0: syntax::Rgb = syntax::rgb!(131, 148, 150);
const BASE1: syntax::Rgb = syntax::rgb!(147, 161, 161);
const BASE3: syntax::Rgb = syntax::rgb!(253, 246, 227);
const YELLOW: syntax::Rgb = syntax::rgb!(181, 137, 0);
const ORANGE: syntax::Rgb = syntax::rgb!(203, 75, 22);
const RED: syntax::Rgb = syntax::rgb!(220, 50, 47);
const BLUE: syntax::Rgb = syntax::rgb!(38, 139, 210);
const CYAN: syntax::Rgb = syntax::rgb!(42, 161, 152);
const GREEN: syntax::Rgb = syntax::rgb!(133, 153, 0);

macro_rules! create_solarized_theme {
    ($theme_name: ident, $fg: expr, $bg: expr, $deemphasized: expr) => {
        pub(crate) struct $theme_name;

        impl syntax::Theme for $theme_name {
            fn default_style(&self) -> syntax::ResolvedStyle {
                syntax::ResolvedStyle {
                    fg_color: $fg,
                    bg_color: $bg,
                    is_bold: false,
                    is_italic: false,
                    is_underline: false,
                }
            }

            fn style(&self, group: syntax::HighlightGroup) -> syntax::Style {
                match group {
                    syntax::HighlightGroup::CtrlFlowKeyword
                    | syntax::HighlightGroup::OtherKeyword => syntax::Style {
                        fg_color: Some(GREEN),
                        bg_color: None,
                        is_bold: false,
                        is_italic: false,
                        is_underline: false,
                    },

                    // ‘Identifiers’ (functions, variables, modules)
                    syntax::HighlightGroup::FunctionDef
                    | syntax::HighlightGroup::FunctionCall
                    | syntax::HighlightGroup::VariableDef
                    | syntax::HighlightGroup::VariableUse
                    | syntax::HighlightGroup::MemberDef
                    | syntax::HighlightGroup::MemberUse
                    | syntax::HighlightGroup::SpecialIdentDef
                    | syntax::HighlightGroup::SpecialIdentUse
                    | syntax::HighlightGroup::SpecialIdentDefSigil
                    | syntax::HighlightGroup::SpecialIdentUseSigil
                    | syntax::HighlightGroup::FunctionParam
                    | syntax::HighlightGroup::ModuleDef
                    | syntax::HighlightGroup::ModuleUse => syntax::Style {
                        fg_color: Some(BLUE),
                        bg_color: None,
                        is_bold: false,
                        is_italic: false,
                        is_underline: false,
                    },

                    // Constants of any kind
                    syntax::HighlightGroup::ConstantDef
                    | syntax::HighlightGroup::ConstantUse
                    | syntax::HighlightGroup::Number
                    | syntax::HighlightGroup::String
                    | syntax::HighlightGroup::StringDelimiter
                    | syntax::HighlightGroup::Character
                    | syntax::HighlightGroup::CharacterDelimiter
                    | syntax::HighlightGroup::Boolean => syntax::Style {
                        fg_color: Some(CYAN),
                        bg_color: None,
                        is_bold: false,
                        is_italic: false,
                        is_underline: false,
                    },

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

                    syntax::HighlightGroup::PreProc
                    | syntax::HighlightGroup::MacroDef
                    | syntax::HighlightGroup::MacroUse => syntax::Style {
                        fg_color: Some(ORANGE),
                        bg_color: None,
                        is_bold: false,
                        is_italic: false,
                        is_underline: false,
                    },

                    // Punctuation gets no highlighting
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

                    syntax::HighlightGroup::Comment | syntax::HighlightGroup::DocComment => {
                        syntax::Style {
                            fg_color: Some($deemphasized),
                            bg_color: None,
                            is_bold: false,
                            is_italic: true,
                            is_underline: false,
                        }
                    }

                    syntax::HighlightGroup::Attribute => syntax::Style {
                        fg_color: Some(GREEN),
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
    };
}

create_solarized_theme!(SolarizedLight, BASE00, BASE3, BASE1);
create_solarized_theme!(SolarizedDark, BASE0, BASE03, BASE01);
