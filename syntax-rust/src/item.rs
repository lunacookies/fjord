use {crate::ParseResult, nom::branch::alt};

mod fn_def;
mod struct_def;
mod trait_def;
mod type_def;
mod use_decl;

use {
    fn_def::parse as fn_def, struct_def::parse as struct_def, trait_def::parse as trait_def,
    type_def::parse as type_def, use_decl::parse as use_decl,
};

pub(crate) fn parse(s: &str) -> ParseResult<'_> {
    alt((trait_def, use_decl, type_def, fn_def, struct_def))(s)
}
