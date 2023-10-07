/*
    Appellation: decanter-derive <library>
    Creator: FL03 <jo3mccain@icloud.com>
*/
extern crate proc_macro;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Hashable)]
pub fn hashable(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let gen = impl_hashable(&ast);

    gen.into()
}

/// This function is used to generate the implementation of the Hashable trait.
fn impl_hashable(ast: &DeriveInput) -> proc_macro2::TokenStream {
    let ident = &ast.ident;
    let res = quote::quote! {
        impl decanter::hash::Hashable for #ident {
            fn hash(&self) -> decanter::hash::H256 {
                decanter::hash::hasher(self.to_string()).into()
            }
        }
    };
    res
}

#[proc_macro_derive(Shash)]
pub fn shash(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let gen = impl_shash(&ast);

    gen.into()
}

/// This function is used to generate the implementation of the Hashable trait.
fn impl_shash(ast: &DeriveInput) -> proc_macro2::TokenStream {
    let ident = &ast.ident;
    let res = quote::quote! {
        impl decanter::hash::Shash for #ident {
            fn hash(&self) -> decanter::hash::H256 {
                decanter::hash::hash_serialize(self).into()
            }
        }
    };
    res
}
