/*
    Appellation: hashes <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Hashes
//!
//!
pub use self::{h160::*, h256::*, hash::*, kinds::*, utils::*};

pub(crate) mod h160;
pub(crate) mod h256;
pub(crate) mod hash;
pub(crate) mod kinds;

/// The H160Hash type is a 20-byte hash.
pub type H160Hash = [u8; 20];
/// The H256Hash type is a 32-byte hash.
pub type H256Hash = [u8; 32];

pub(crate) mod utils {
    use super::*;

    pub fn digest_to_hash<const N: usize>(hash: impl AsRef<[u8]>) -> [u8; N] {
        let mut raw_hash: [u8; N] = [0; N];
        raw_hash[0..N].copy_from_slice(hash.as_ref());
        raw_hash
    }

    pub fn hash(data: impl AsRef<[u8]>) -> H256 {
        let hash = blake3::hash(data.as_ref());
        H256(digest_to_hash::<32>(hash.as_bytes()))
    }
}
