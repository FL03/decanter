/*
    Appellation: container <mod>
    Creator: FL03 <jo3mccain@icloud.com>
*/
use crate::internal::{attr, Ctxt};

#[derive(Clone)]
pub struct Container {
    /// Attributes on the structure, parsed for Serde.
    pub attrs: attr::Container,
    /// The struct or enum name (without generics).
    pub ident: syn::Ident,
}

impl Container {
    pub fn new(attrs: attr::Container, ident: syn::Ident) -> Self {
        Self { attrs, ident }
    }

    pub fn from_ast(cx: &Ctxt, item: &syn::DeriveInput) -> Option<Self> {
        let ident = item.ident.clone();
        let attrs = attr::Container::from_ast(cx, item);

        let cont = Self::new(attrs, ident);
        Some(cont)
    }

    pub fn attrs(&self) -> &attr::Container {
        &self.attrs
    }

    pub fn ident(&self) -> &syn::Ident {
        &self.ident
    }
}
