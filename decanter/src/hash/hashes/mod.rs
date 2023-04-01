/*
    Appellation: hashes <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use self::{h160::*, h256::*};

mod h160;
mod h256;

/// The H160Hash type is a 20-byte hash.
pub type H160Hash = [u8; 20];
/// The H256Hash type is a 32-byte hash.
pub type H256Hash = [u8; 32];
