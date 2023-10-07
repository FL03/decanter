/*
    Appellation: attr <mod>
    Creator: FL03 <jo3mccain@icloud.com>
*/
pub use self::{attr::*, container::*, utils::*};

pub(crate) mod attr;
pub(crate) mod container;

pub(crate) mod utils {
    use crate::internal::*;

    use syn::meta::ParseNestedMeta;

    pub fn get_lit_str(
        cx: &Ctxt,
        attr_name: Symbol,
        meta: &ParseNestedMeta,
    ) -> syn::Result<Option<syn::LitStr>> {
        get_lit_str2(cx, attr_name, attr_name, meta)
    }

    pub fn get_lit_str2(
        cx: &Ctxt,
        attr_name: Symbol,
        meta_item_name: Symbol,
        meta: &ParseNestedMeta,
    ) -> syn::Result<Option<syn::LitStr>> {
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
            let suffix = lit.suffix();
            if !suffix.is_empty() {
                cx.error_spanned_by(
                    lit,
                    format!("unexpected suffix `{}` on string literal", suffix),
                );
            }
            Ok(Some(lit.clone()))
        } else {
            cx.error_spanned_by(
                expr,
                format!(
                    "expected serde {} attribute to be a string: `{} = \"...\"`",
                    attr_name, meta_item_name
                ),
            );
            Ok(None)
        }
    }

    pub fn parse_lit_into_path(
        cx: &Ctxt,
        attr_name: Symbol,
        meta: &ParseNestedMeta,
    ) -> syn::Result<Option<syn::Path>> {
        let string = match get_lit_str(cx, attr_name, meta)? {
            Some(string) => string,
            None => return Ok(None),
        };

        Ok(match string.parse() {
            Ok(path) => Some(path),
            Err(_) => {
                cx.error_spanned_by(
                    &string,
                    format!("failed to parse path: {:?}", string.value()),
                );
                None
            }
        })
    }
}
