/*
    Appellation: hashes <module>
    Contributors: FL03 <jo3mccain@icloud.com>
*/
pub use self::{h160::*, h256::*, hash::*};

mod h160;
mod h256;
mod hash;

/// The H160Hash type is a 20-byte hash.
pub type H160Hash = [u8; 20];
/// The H256Hash type is a 32-byte hash.
pub type H256Hash = [u8; 32];

pub(crate) mod utils {}
