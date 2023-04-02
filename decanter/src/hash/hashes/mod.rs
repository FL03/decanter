/*
    Appellation: hashes <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use self::{h160::*, h256::*};

mod h160;
mod h256;

use crate::GenericHash;

/// The H160Hash type is a 20-byte hash.
pub type H160Hash = [u8; 20];
/// The H256Hash type is a 32-byte hash.
pub type H256Hash = [u8; 32];

///
pub trait Hasher {
    fn hash(data: impl AsRef<[u8]>) -> GenericHash {
        blake3::hash(data.as_ref()).as_bytes().to_owned().into()
    }
    fn hash_to_deg(data: impl AsRef<[u8]>, deg: Option<usize>) -> GenericHash {
        let hs = Self::hash(data);
        let mut res: GenericHash = hs;
        for _ in 0..deg.unwrap_or(1) {
            res = Self::hash(res);
        }
        res
    }
}

pub trait UpdatableHasher: Hasher {
    fn update(&mut self, data: impl AsRef<[u8]>);
    fn finalize(&mut self) -> GenericHash;
}
