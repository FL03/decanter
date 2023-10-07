/*
    Appellation: attr <mod>
    Creator: FL03 <jo3mccain@icloud.com>
*/
#![allow(dead_code)]
use crate::internal::{Ctxt, Symbol};

use proc_macro2::TokenStream;
use quote::ToTokens;

pub trait Attribute<T> {
    fn name(&self) -> Symbol;

    fn tokens(&self) -> &TokenStream;

    fn get(&self) -> Option<T>;

    fn get_with_tokens(self) -> Option<(TokenStream, T)>;
}

#[derive(Clone)]
pub struct Attr<'c, T> {
    cx: &'c Ctxt,
    name: Symbol,
    tokens: TokenStream,
    value: Option<T>,
}

impl<'c, T> Attr<'c, T> {
    pub fn none(cx: &'c Ctxt, name: Symbol) -> Self {
        Attr {
            cx,
            name,
            tokens: TokenStream::new(),
            value: None,
        }
    }

    pub fn set<A: ToTokens>(&mut self, obj: A, value: T) {
        let tokens = obj.into_token_stream();

        if self.value.is_some() {
            let msg = format!("duplicate attribute `{}`", self.name);
            self.cx.error_spanned_by(tokens, msg);
        } else {
            self.tokens = tokens;
            self.value = Some(value);
        }
    }

    pub fn set_opt<A: ToTokens>(&mut self, obj: A, value: Option<T>) {
        if let Some(value) = value {
            self.set(obj, value);
        }
    }

    pub fn set_if_none(&mut self, value: T) {
        if self.value.is_none() {
            self.value = Some(value);
        }
    }

    pub fn get(self) -> Option<T> {
        self.value
    }

    pub fn get_with_tokens(self) -> Option<(TokenStream, T)> {
        match self.value {
            Some(v) => Some((self.tokens, v)),
            None => None,
        }
    }
}

pub struct BoolAttr<'c>(pub Attr<'c, ()>);

impl<'c> BoolAttr<'c> {
    pub fn attr(&self) -> Attr<'c, ()> {
        self.0.clone()
    }
    pub fn none(cx: &'c Ctxt, name: Symbol) -> Self {
        BoolAttr(Attr::none(cx, name))
    }

    pub fn set_true<A: ToTokens>(&mut self, obj: A) {
        self.0.set(obj, ());
    }

    pub fn get(&self) -> bool {
        self.0.value.is_some()
    }
}

pub struct VecAttr<'c, T> {
    cx: &'c Ctxt,
    name: Symbol,
    first_dup_tokens: TokenStream,
    values: Vec<T>,
}

impl<'c, T> VecAttr<'c, T> {
    pub fn none(cx: &'c Ctxt, name: Symbol) -> Self {
        VecAttr {
            cx,
            name,
            first_dup_tokens: TokenStream::new(),
            values: Vec::new(),
        }
    }

    pub fn insert<A: ToTokens>(&mut self, obj: A, value: T) {
        if self.values.len() == 1 {
            self.first_dup_tokens = obj.into_token_stream();
        }
        self.values.push(value);
    }

    pub fn at_most_one(mut self) -> Option<T> {
        if self.values.len() > 1 {
            let dup_token = self.first_dup_tokens;
            let msg = format!("duplicate serde attribute `{}`", self.name);
            self.cx.error_spanned_by(dup_token, msg);
            None
        } else {
            self.values.pop()
        }
    }

    pub fn get(self) -> Vec<T> {
        self.values
    }
}
