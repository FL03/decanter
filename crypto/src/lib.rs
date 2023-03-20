/*
    Appellation: decanter-crypto <library>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
#[doc(inline)]
pub use self::{hash::*, keys::*, primitives::*, utils::*};

pub(crate) mod hash;
pub(crate) mod keys;
pub(crate) mod primitives;
pub(crate) mod utils;

///
pub trait Hashable: std::fmt::Display {
    fn hash(&self) -> H256;
}

///
pub trait HashableExt: Hashable {
    fn hasher(&self, deg: Option<usize>) -> H256 {
        let s: H256 = hasher(&self).into();

        let mut res: H256 = s;
        for _ in 0..deg.unwrap_or(1) {
            res = hasher(&res.clone()).into()
        }
        res
    }
}
