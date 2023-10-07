/*
    Appellation: container <mod>
    Creator: FL03 <jo3mccain@icloud.com>
*/
use super::BoolAttr;
use crate::internal::*;
use quote::ToTokens;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum HashType {
    #[default]
    Serde,
    String,
}

impl HashType {
    pub fn decide(cx: &Ctxt, _item: &syn::DeriveInput, serde: BoolAttr, string: BoolAttr) -> Self {
        match (
            serde.attr().get_with_tokens(),
            string.attr().get_with_tokens(),
        ) {
            (None, Some(_)) => Self::String,
            (Some((tokens, _)), Some((_, _))) => {
                cx.error_spanned_by(tokens, "conflicitng attributes");
                Self::Serde
            }
            _ => Self::Serde,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Container {
    pub non_exhaustive: bool,
    pub uses: HashType,
}

impl Container {
    pub fn new(non_exhaustive: bool, uses: HashType) -> Self {
        Self {
            non_exhaustive,
            uses,
        }
    }
    /// Extract out the `#[decanter(...)]` attributes from an item.
    pub fn from_ast(cx: &Ctxt, item: &syn::DeriveInput) -> Self {
        let mut serde = BoolAttr::none(cx, SERDE);
        let mut string = BoolAttr::none(cx, STRING);
        // properties
        let mut non_exhaustive = false;

        for attr in &item.attrs {
            if attr.path() != DECANTER {
                non_exhaustive |=
                    matches!(&attr.meta, syn::Meta::Path(path) if path == NON_EXHAUSTIVE);
                continue;
            }

            if let syn::Meta::List(meta) = &attr.meta {
                if meta.tokens.is_empty() {
                    continue;
                }
            }
            if let Err(err) = attr.parse_nested_meta(|meta| {
                if meta.path == SERDE {
                    // #[decanter(serde)]
                    serde.set_true(meta.path);
                } else if meta.path == STRING {
                    // #[decanter(string)]
                    string.set_true(meta.path);
                } else {
                    let path = meta.path.to_token_stream().to_string().replace(' ', "");
                    return Err(
                        meta.error(format_args!("unknown serde container attribute `{}`", path))
                    );
                }
                Ok(())
            }) {
                cx.syn_error(err);
            }
        }

        Self::new(non_exhaustive, HashType::decide(cx, item, serde, string))
    }

    pub fn uses(&self) -> HashType {
        self.uses
    }
}

// impl Parse for Container {
//     fn parse(input: ParseStream) -> SynResult<Self> {
//         let content;
//         syn::parenthesized!(content in input);
//         let serde = content.parse()?;
//         content.parse::<Token![,]>()?;
//         let string = content.parse()?;
//         content.parse::<Token![,]>()?;
//         Ok(Self::new(serde, string))
//     }
// }
