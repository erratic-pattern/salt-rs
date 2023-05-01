use std::path::PathBuf;

use glob::GlobError;
use syn::{meta::ParseNestedMeta, LitStr, Meta};

use crate::{error::new_spanned_err, symbol::MODULES};

#[derive(Default)]
pub struct BinMeta {
    modules: Option<Vec<PathBuf>>,
}

impl BinMeta {
    // Constructs an empty MainInput
    #[allow(dead_code)]

    fn new() -> Self {
        Self { modules: None }
    }
}

pub fn parse_bin_meta(meta: Meta) -> syn::Result<BinMeta> {
    parse_nested_meta_with_default(meta, |parsed: &mut BinMeta, meta| {
        if meta.path == MODULES {
            parsed.modules = Some(parse_modules_meta(&meta)?);
        }
        Ok(())
    })
}

#[derive(Default)]
pub struct MainMeta {
    modules: Option<Vec<PathBuf>>,
}

impl MainMeta {
    // Constructs an empty MainInput
    #[allow(dead_code)]
    fn new() -> Self {
        Self { modules: None }
    }
}

pub fn parse_main_meta(meta: Meta) -> syn::Result<MainMeta> {
    parse_nested_meta_with_default(meta, |parsed: &mut MainMeta, meta| {
        if meta.path == MODULES {
            parsed.modules = Some(parse_modules_meta(&meta)?);
        }
        Ok(())
    })
}

impl From<BinMeta> for MainMeta {
    fn from(meta: BinMeta) -> Self {
        Self {
            modules: meta.modules,
        }
    }
}

fn parse_modules_meta(meta: &ParseNestedMeta) -> syn::Result<Vec<PathBuf>> {
    parse_modules_glob(get_lit_str(meta)?)
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

// Helper to parse Meta.
// * Meta::Path will return default()
// * Meta::List will use the provided callback with parse_nested_meta
// * Meta::NameValue will error
fn parse_nested_meta_with_default<T, F>(meta: Meta, mut logic: F) -> syn::Result<T>
where
    T: Default,
    F: FnMut(&mut T, ParseNestedMeta) -> syn::Result<()>,
{
    let mut parsed = T::default();
    if let Meta::Path { .. } = meta {
        return Ok(parsed);
    }
    let meta = meta.require_list()?;
    meta.parse_nested_meta(|meta| logic(&mut parsed, meta))?;
    Ok(parsed)
}
