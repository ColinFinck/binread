///! Utilities for helping sanitize macro
use proc_macro2::{Ident, Span, TokenStream};
use quote::{ToTokens, TokenStreamExt, format_ident, quote};

macro_rules! ident_str {
    () => {};

    ($vis:vis $ident:ident = $path:expr; $($tail:tt)*) => {
        ident_str!($vis $ident = $path);
        ident_str!($($tail)*);
    };

    ($vis:vis $ident:ident = $path:expr) => {
        $vis const $ident: IdentStr = IdentStr::new($path);
    };
}

macro_rules! from_crate {
    ($path:path) => { concat!("binread::", stringify!($path)) };
}

macro_rules! from_trait {
    () => { from_crate!(BinRead) };
    ($path:path) => { concat!("binread::BinRead::", stringify!($path)) };
}

ident_str! {
    pub(super) TRAIT_NAME = from_trait!();
    pub(super) BIN_ERROR = from_crate!(Error);
    pub(super) OPTIONS = from_crate!(ReadOptions);
    pub(super) READ_TRAIT = from_crate!(io::Read);
    pub(super) SEEK_TRAIT = from_crate!(io::Seek);
    pub(super) SEEK_FROM = from_crate!(io::SeekFrom);
    pub(super) BIN_RESULT = from_crate!(BinResult);
    pub(super) ENDIAN_ENUM = from_crate!(Endian);
    pub(super) READ_METHOD = from_trait!(read_options);
    pub(super) AFTER_PARSE = from_trait!(after_parse);
    pub(super) READER = "__binread_generated_var_reader";
    pub(super) OPT = "__binread_generated_var_options";
    pub(super) ARGS = "__binread_generated_var_arguments";
    pub(super) SAVED_POSITION = "__binread_generated_saved_position";
    pub(super) ASSERT_MAGIC = from_crate!(error::magic);
    pub(super) ASSERT = from_crate!(error::assert);
    pub(super) WRITE_START_STRUCT = from_crate!(binary_template::write_start_struct);
    pub(super) WRITE_END_STRUCT = from_crate!(binary_template::write_end_struct);
    pub(super) WRITE_COMMENT = from_crate!(binary_template::write_comment);
    pub(super) READ_METHOD_NOP = from_crate!(error::nop3);
    pub(super) READ_METHOD_DEFAULT = from_crate!(error::nop3_default);
    pub(super) AFTER_PARSE_NOP = from_crate!(error::nop5);
    pub(super) AFTER_PARSE_TRY = from_crate!(error::try_after_parse);
    pub(super) AFTER_PARSE_IDENTITY = from_crate!(error::identity_after_parse);
    pub(super) TRY_CONVERSION = from_crate!(error::try_conversion);
    pub(super) TEMP = "__binread_temp";
    pub(super) POS = "__binread_generated_position_temp";
    pub(super) ERROR_BASKET = "__binread_generated_error_basket";
}

pub(crate) fn make_ident(ident: &Ident, kind: &str) -> Ident {
    format_ident!("__binread_generated_{}_{}", kind, ident.to_string())
}

/// A string wrapper that converts the str to a $path `TokenStream`, allowing
/// for constant-time idents that can be shared across threads
#[derive(Debug, Clone, Copy)]
pub struct IdentStr(&'static str);

impl IdentStr {
    pub(crate) const fn new(str: &'static str) -> Self {
        IdentStr(str)
    }
}

impl ToTokens for IdentStr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let idents = self.0.split("::").map(|ident| {
            Ident::new(ident, Span::call_site())
        });
        tokens.append_separated(idents, quote!(::));
    }
}
