/*
    Appellation: hash <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::{hashes::*, iter::*};

pub(crate) mod hashes;
pub(crate) mod iter;

use crate::{hasher, GenericHash, Hashable};

///
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Hash(pub GenericHash);

impl Hash {
    pub fn new<T>(data: T) -> Self
    where
        T: ToString,
    {
        Self(hasher(&data))
    }
}

impl Hashable for Hash {
    fn hash(&self) -> H256 {
        self.0.as_slice().to_owned().into()
    }
}

impl std::fmt::Display for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:x}", self.0)
    }
}

impl From<GenericHash> for Hash {
    fn from(data: GenericHash) -> Self {
        Self(data)
    }
}

impl From<&Hash> for Hash {
    fn from(data: &Hash) -> Self {
        Self::new(data.clone())
    }
}

impl From<Hash> for Vec<u8> {
    fn from(hash: Hash) -> Vec<u8> {
        hash.0.as_slice().to_owned()
    }
}

impl From<Hash> for H256 {
    fn from(hash: Hash) -> H256 {
        hash.hash()
    }
}
