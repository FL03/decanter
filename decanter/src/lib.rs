/*
    Appellation: decanter <library>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/

pub use self::{primitives::*, specs::*, utils::*};
#[doc(inline)]
#[cfg(feature = "crypto")]
pub use decanter_crypto as crypto;
#[cfg(feature = "derive")]
pub use decanter_derive::*;

pub mod hash;

mod primitives;
mod specs;
mod utils;

pub mod prelude {
    pub use blake3;

    pub use super::*;

    pub use super::hash::*;

    #[cfg(feature = "crypto")]
    pub use super::crypto::*;
}
