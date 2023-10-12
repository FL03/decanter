/*
    Appellation: hasher <module>
    Contributors: FL03 <jo3mccain@icloud.com>
*/
use super::{Hash, H256};

pub trait Hasher {
    type Hash: Hash;

    fn finalize(&self) -> Self::Hash;

    fn hash(data: impl AsRef<[u8]>) -> Self::Hash;

    fn update(&mut self, data: impl AsRef<[u8]>) -> &mut Self;
}

impl Hasher for blake3::Hasher {
    type Hash = H256;

    fn finalize(&self) -> Self::Hash {
        blake3::Hasher::finalize(&self).into()
    }

    fn hash(data: impl AsRef<[u8]>) -> Self::Hash {
        blake3::hash(data.as_ref()).into()
    }

    fn update(&mut self, data: impl AsRef<[u8]>) -> &mut Self {
        self.update(data.as_ref())
    }
}
