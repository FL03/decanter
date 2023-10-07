/*
    Appellation: symbol <mod>
    Creator: FL03 <jo3mccain@icloud.com>
*/
pub use self::symbols::*;

use syn::{Ident, Path};

#[derive(Copy, Clone)]
pub struct Symbol(&'static str);

impl Symbol {
    pub fn new(sym: &'static str) -> Self {
        Symbol(sym)
    }
}

impl AsRef<str> for Symbol {
    fn as_ref(&self) -> &str {
        self.0
    }
}

impl std::fmt::Display for Symbol {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str(self.0)
    }
}

impl<'a> PartialEq<Symbol> for &'a Ident {
    fn eq(&self, word: &Symbol) -> bool {
        *self == word.0
    }
}

impl PartialEq<Symbol> for Path {
    fn eq(&self, word: &Symbol) -> bool {
        self.is_ident(word.0)
    }
}

impl<'a> PartialEq<Symbol> for &'a Path {
    fn eq(&self, word: &Symbol) -> bool {
        self.is_ident(word.0)
    }
}

pub(crate) mod symbols {
    use super::Symbol;

    pub const DECANTER: Symbol = Symbol("dec");

    pub const NON_EXHAUSTIVE: Symbol = Symbol("non_exhaustive");

    pub const SERDE: Symbol = Symbol("serde");

    pub const STRING: Symbol = Symbol("string");
}
