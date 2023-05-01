use meta::{parse_bin_meta, parse_main_meta, BinMeta, MainMeta};

use quote::ToTokens;

use syn::{parse_macro_input, parse_quote, Item, ItemFn, Meta};

mod error;
mod meta;
mod symbol;

#[proc_macro_attribute]
pub fn bin(
    attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    match parse_bin_meta(parse_macro_input!(attr as Meta)) {
        Err(err) => err.into_compile_error().into(),
        Ok(meta) => {
            let bin_file = parse_macro_input!(input as syn::File);
            gen_salt_bin(meta, bin_file).to_token_stream().into()
        }
    }
}

#[proc_macro_attribute]
pub fn main(
    attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    match parse_main_meta(parse_macro_input!(attr as Meta)) {
        Err(err) => err.into_compile_error().into(),
        Ok(meta) => {
            let main = parse_macro_input!(input as ItemFn);
            wrap_main(meta, main).to_token_stream().into()
        }
    }
}

fn gen_salt_bin(meta: BinMeta, mut bin_file: syn::File) -> syn::File {
    bin_file.items.push(Item::Fn(gen_salt_main(meta)));
    bin_file
}

fn gen_salt_main(meta: BinMeta) -> ItemFn {
    let new_main = parse_quote! {
        pub fn main() { }
    };
    wrap_main(meta.into(), new_main)
}

fn wrap_main(_meta: MainMeta, main: ItemFn) -> ItemFn {
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
