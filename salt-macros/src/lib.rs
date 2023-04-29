use proc_macro2::Span;
use quote::{quote_spanned, ToTokens};
use syn::{ItemFn, ItemMod};

struct SaltBin {
    main: SaltMainFn,
    modules: Vec<SaltModule>,
}

impl ToTokens for SaltBin {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        todo!()
    }
}

struct SaltMainFn;

impl ToTokens for SaltMainFn {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        todo!()
    }
}
struct SaltModule;

impl ToTokens for SaltModule {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        todo!()
    }
}

#[proc_macro_attribute]
pub fn main(
    attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    if let Ok(input) = syn::parse::<ItemMod>(input.clone()) {
        parse_salt_bin(input).to_token_stream().into()
    } else if let Ok(input) = syn::parse::<ItemFn>(input) {
        parse_salt_main(input).to_token_stream().into()
    } else {
        quote_spanned! { Span::call_site() => compile_error!("Invalid salt::main definition: must be a module or main funciton") }.into()
    }
}

fn parse_salt_bin(module: ItemMod) -> SaltBin {
    todo!()
}

fn parse_salt_main(main: ItemFn) -> SaltMainFn {
    todo!()
}
