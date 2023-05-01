use std::fmt::Display;

use quote::ToTokens;

pub fn new_spanned_err<T>(tokens: impl ToTokens, message: impl Display) -> syn::Result<T> {
    Err(syn::Error::new_spanned(tokens, message))
}
