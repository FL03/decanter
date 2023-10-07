/*
    Appellation: decanter-derive <library>
    Creator: FL03 <jo3mccain@icloud.com>
*/
extern crate proc_macro;
extern crate quote;
extern crate syn;

pub(crate) mod internal;

use internal::*;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, DeriveInput, Ident, Token};

type SynResult<T = ()> = std::result::Result<T, syn::Error>;

struct Decanter {
    serde: Option<Ident>,
    string: Option<Ident>,
}

impl Parse for Decanter {
    fn parse(input: ParseStream) -> Result<Self> {
        let serde = input.parse().unwrap_or(None);
        input.parse::<Token![,]>()?;
        let string = input.parse().unwrap_or(None);

        Ok(Decanter { serde, string })
    }
}

#[proc_macro_derive(Hashable)]
pub fn hashable(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    
    let gen = impl_hashable(&ast);

    gen.into()
}

/// This function is used to generate the implementation of the Hashable trait.
fn impl_hashable(ast: &DeriveInput) -> proc_macro2::TokenStream {
    let ident = &ast.ident;
    hash_string_body(ident)
}

#[proc_macro_derive(Dash, attributes(dec))]
pub fn shash(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let gen = impl_shash(&ast);

    gen.expect("").into()
}

/// This function is used to generate the implementation of the Hashable trait.
fn impl_shash(ast: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let ident = &ast.ident;
    let ctxt = Ctxt::new();
    let cont = match ast::Container::from_ast(&ctxt, ast) {
        Some(cont) => cont,
        None => return Err(ctxt.check().unwrap_err()),
    };
    let attr = cont.attrs;
    let res = match attr.uses() {
        attr::HashType::Serde => {
            hash_serde_body(ident)
        }
        attr::HashType::String => {
            hash_string_body(ident)
        }
    };
    Ok(res)
}


/// This function is used to generate the implementation of the Hashable trait.
fn hash_serde_body(ident: &Ident) -> proc_macro2::TokenStream {
    quote! {
        impl decanter::hash::Hashable for #ident {
            fn hash(&self) -> decanter::hash::H256 {
                decanter::hash::hash_serialize(&self)
            }
        }
    }
}

// This function is used to generate the implementation of the Hashable trait.
fn hash_string_body(ident: &Ident) -> proc_macro2::TokenStream {
    quote! {
        impl decanter::hash::Hashable for #ident {
            fn hash(&self) -> decanter::hash::H256 {
                decanter::hash::hasher(self.to_string()).into()
            }
        }
    }
}
