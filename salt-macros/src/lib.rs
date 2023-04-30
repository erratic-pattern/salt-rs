use std::{fmt::Display, path::PathBuf};

use glob::GlobError;
use proc_macro2::Span;
use quote::{quote_spanned, ToTokens};
use symbol::*;
use syn::{meta::ParseNestedMeta, parse_macro_input, parse_quote, Item, ItemFn, LitStr, Meta};
use thiserror::Error;

mod symbol;

#[derive(Error, Debug)]
enum MacroError {
    #[error("Invalid attribute syntax in main: {0}")]
    InvalidMainInput(String),
}

#[derive(Default)]
struct MainMeta {
    modules: Vec<PathBuf>,
}

impl MainMeta {
    // Constructs an empty MainInput
    fn new() -> Self {
        MainMeta {
            modules: Vec::new(),
        }
    }
}

fn parse_main_meta(meta: Meta) -> syn::Result<MainMeta> {
    if let Meta::Path { .. } = meta {
        return Ok(MainMeta::default());
    }
    let meta = meta.require_list()?;
    let mut main_attr = MainMeta::new();
    meta.parse_nested_meta(|meta| {
        if meta.path == MODULES {
            main_attr.modules = parse_modules_glob(get_lit_str(&meta)?)?
        }
        Ok(())
    })?;
    Ok(main_attr)
    // _ => Err(MacroError::InvalidMainInput("Invalid attribute syntax. Expected a list of key/value pairs. Example: main(modules = \"**/*.rs\")")),
}

fn parse_modules_glob(modules_glob: LitStr) -> syn::Result<Vec<PathBuf>> {
    match glob::glob(&modules_glob.value()) {
        Err(err) => new_spanned_err(modules_glob, err),
        Ok(paths) => paths
            .collect::<Result<Vec<PathBuf>, GlobError>>()
            .or_else(|err| new_spanned_err(modules_glob, err)),
    }
}

fn get_lit_str(meta: &ParseNestedMeta) -> syn::Result<syn::LitStr> {
    let expr: syn::Expr = meta.value()?.parse()?;
    let mut value = &expr;
    while let syn::Expr::Group(e) = value {
        value = &e.expr;
    }
    if let syn::Expr::Lit(syn::ExprLit {
        lit: syn::Lit::Str(lit),
        ..
    }) = value
    {
        Ok(lit.clone())
    } else {
        new_spanned_err(expr, "expected attribute to be a string".to_string())
    }
}

fn new_spanned_err<T>(tokens: impl ToTokens, message: impl Display) -> syn::Result<T> {
    Err(syn::Error::new_spanned(tokens, message))
}

#[proc_macro_attribute]
pub fn main(
    attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    match parse_main_meta(parse_macro_input!(attr as Meta)) {
        Err(err) => err.into_compile_error().into(),
        Ok(meta) => {
            if let Ok(bin_file) = syn::parse::<syn::File>(input.clone()) {
                gen_salt_bin(meta, bin_file).to_token_stream().into()
            } else if let Ok(main) = syn::parse::<ItemFn>(input) {
                wrap_main(main).to_token_stream().into()
            } else {
                quote_spanned! { Span::call_site() => compile_error!("Invalid salt::main invocation: must be attached to source file or main funciton") }.into()
            }
        }
    }
}

fn gen_salt_bin(meta: MainMeta, mut bin_file: syn::File) -> syn::File {
    bin_file.items.push(Item::Fn(gen_salt_main()));
    bin_file
}

fn gen_salt_main() -> ItemFn {
    let new_main = parse_quote! {
        pub fn main() { }
    };
    wrap_main(new_main)
}

fn wrap_main(main: ItemFn) -> ItemFn {
    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = main;
    parse_quote! {
        #(#attrs)*
        #vis #sig {
            #block
        }
    }
}
