/*
    Appellation: decanter <library>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
*/
//! # Decanter
//!
//! Decanter is a collection of cryptographic primitives and utilities for the Rust programming language.
//! As of now, Decanter is a work in progress and is not ready for production use.
//! Additionally, Decanter provides several hashing utilities.

pub use self::utils::*;

#[cfg(feature = "crypto")]
pub use decanter_crypto as crypto;
#[cfg(feature = "derive")]
pub use decanter_derive::*;
#[cfg(feature = "macros")]
pub use decanter_macros::*;

pub mod hash;
pub mod specs;

pub(crate) mod utils;

pub mod prelude {
    pub use blake3;

    pub use crate::hash::*;
    pub use crate::specs::*;
    pub use crate::utils::*;

    #[cfg(feature = "crypto")]
    pub use crate::crypto::*;
    #[cfg(feature = "derive")]
    pub use decanter_derive::*;
    #[cfg(feature = "macros")]
    pub use decanter_macros::*;
}
