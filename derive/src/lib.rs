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
use syn::{parse_macro_input, DeriveInput, Result};

#[proc_macro_derive(Hashable, attributes(decanter))]
pub fn hashable(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let gen = impl_hashable(&ast);

    gen.expect("").into()
}

/// This function is used to generate the implementation of the Hashable trait.
fn impl_hashable(ast: &DeriveInput) -> Result<proc_macro2::TokenStream> {
    let ctxt = Ctxt::new();
    let cont = match ast::Container::from_ast(&ctxt, ast) {
        Some(cont) => cont,
        None => return Err(ctxt.check().unwrap_err()),
    };
    ctxt.check()?;
    derive_hash(&cont)
}

fn derive_hash(cont: &ast::Container) -> Result<proc_macro2::TokenStream> {
    let attr = cont.attrs();
    let res = match attr.uses() {
        attr::HashType::Serde => hash_serde_body(&cont),
        attr::HashType::String => hash_string_body(&cont),
    };
    Ok(res)
}

/// This function is used to generate the implementation of the Hashable trait.
fn hash_serde_body(cont: &ast::Container) -> proc_macro2::TokenStream {
    let attr = cont.attrs();
    let decanter = attr.crate_path();
    let ident = cont.ident();
    quote! {
        impl #decanter::hash::Hashable for #ident {
            fn hash(&self) -> #decanter::hash::H256 {
                #decanter::hash::hash_serialize(&self)
            }
        }
    }
}

// This function is used to generate the implementation of the Hashable trait.
fn hash_string_body(cont: &ast::Container) -> proc_macro2::TokenStream {
    let attr = cont.attrs();
    let decanter = attr.crate_path();
    let ident = cont.ident();

    quote! {
        impl #decanter::hash::Hashable for #ident {
            fn hash(&self) -> #decanter::hash::H256 {
                #decanter::hash::hasher(self.to_string()).into()
            }
        }
    }
}
