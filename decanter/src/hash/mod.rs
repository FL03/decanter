/*
    Appellation: hash <module>
    Contributors: FL03 <jo3mccain@icloud.com>
*/
//! # Hash
//!
//! The hash module provides a generic hash function wrapper around blake3
//!
pub use self::{hashes::*, iter::*, utils::*};

mod hashes;
mod iter;

use generic_array::GenericArray;
use typenum::{
    bit::{B0, B1},
    uint::{UInt, UTerm},
};

///
pub type GenericHashOutput = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>;
/// [GenericHash] is a generic hash type
pub type GenericHash<T = u8, Output = GenericHashOutput> = GenericArray<T, Output>;

pub type BoxHash = Box<dyn Hash>;

pub struct A(Box<dyn Hash>);

pub trait Hash: AsRef<[u8]> {
    fn as_vec(&self) -> Vec<u8> {
        self.as_ref().to_vec()
    }

    fn hash<H: Hasher<Hash = Self>>(&self, h: &mut H) -> Self
    where
        Self: Sized,
    {
        h.update(self.as_vec()).finalize()
    }
}

pub trait SizedHash: Hash {
    const SIZE: usize = 32;

    fn size(&self) -> usize;
}

// impl Hash for blake3::Hash {
//     fn as_vec(&self) -> Vec<u8> {
//         self.as_bytes().to_vec()
//     }
// }

impl<T> Hash for T where T: AsRef<[u8]> {}

impl<const N: usize> SizedHash for [u8; N] {
    const SIZE: usize = N;

    fn size(&self) -> usize {
        N
    }
}

impl SizedHash for H160 {
    const SIZE: usize = 20;

    fn size(&self) -> usize {
        20
    }
}

impl SizedHash for H256 {
    const SIZE: usize = 32;

    fn size(&self) -> usize {
        32
    }
}

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

/// [Hashable] is a trait that defines a hashable object
pub trait Hashable {
    fn hash(&self) -> H256;
}

impl Hashable for char {
    fn hash(&self) -> H256 {
        blake3::hash(self.to_string().as_bytes()).into()
    }
}

impl Hashable for String {
    fn hash(&self) -> H256 {
        blake3::hash(self.to_string().as_bytes()).into()
    }
}

impl Hashable for &str {
    fn hash(&self) -> H256 {
        blake3::hash(self.to_string().as_bytes()).into()
    }
}

impl Hashable for u8 {
    fn hash(&self) -> H256 {
        blake3::hash(self.to_string().as_bytes()).into()
    }
}

impl Hashable for u16 {
    fn hash(&self) -> H256 {
        blake3::hash(self.to_string().as_bytes()).into()
    }
}

impl Hashable for u32 {
    fn hash(&self) -> H256 {
        blake3::hash(self.to_string().as_bytes()).into()
    }
}

impl Hashable for u64 {
    fn hash(&self) -> H256 {
        blake3::hash(self.to_string().as_bytes()).into()
    }
}

impl Hashable for u128 {
    fn hash(&self) -> H256 {
        blake3::hash(self.to_string().as_bytes()).into()
    }
}

impl Hashable for usize {
    fn hash(&self) -> H256 {
        blake3::hash(self.to_string().as_bytes()).into()
    }
}

impl Hashable for i8 {
    fn hash(&self) -> H256 {
        blake3::hash(self.to_string().as_bytes()).into()
    }
}

impl Hashable for i16 {
    fn hash(&self) -> H256 {
        blake3::hash(self.to_string().as_bytes()).into()
    }
}

impl Hashable for i32 {
    fn hash(&self) -> H256 {
        blake3::hash(self.to_string().as_bytes()).into()
    }
}

impl Hashable for i64 {
    fn hash(&self) -> H256 {
        blake3::hash(self.to_string().as_bytes()).into()
    }
}

impl Hashable for i128 {
    fn hash(&self) -> H256 {
        blake3::hash(self.to_string().as_bytes()).into()
    }
}

impl Hashable for isize {
    fn hash(&self) -> H256 {
        blake3::hash(self.to_string().as_bytes()).into()
    }
}

impl Hashable for bool {
    fn hash(&self) -> H256 {
        blake3::hash(self.to_string().as_bytes()).into()
    }
}

impl Hashable for f32 {
    fn hash(&self) -> H256 {
        blake3::hash(self.to_string().as_bytes()).into()
    }
}

impl Hashable for f64 {
    fn hash(&self) -> H256 {
        blake3::hash(self.to_string().as_bytes()).into()
    }
}

pub(crate) mod utils {
    use super::{GenericHash, H256};
    use serde::Serialize;

    pub fn concat_b3(left: blake3::Hash, right: Option<blake3::Hash>) -> blake3::Hash {
        let mut concatenated: Vec<u8> = left.as_bytes().to_vec();

        match right {
            Some(right_node) => {
                let mut right_node_clone: Vec<u8> = right_node.as_bytes().to_vec();
                concatenated.append(&mut right_node_clone);
                blake3::hash(&concatenated)
            }
            None => left.clone(),
        }
    }

    pub fn concat_hashes(a: &impl AsRef<[u8]>, b: &impl AsRef<[u8]>) -> GenericHash {
        let mut hasher = blake3::Hasher::new();
        hasher.update(a.as_ref());
        hasher.update(b.as_ref());
        hasher.finalize().as_bytes().to_owned().into()
    }

    pub fn generate_random_hash(n: Option<usize>) -> Vec<u8> {
        (0..n.unwrap_or(32)).map(|_| rand::random::<u8>()).collect()
    }

    /// hasher implements a generic hash function wrapper around blake3
    pub fn hasher(data: impl AsRef<[u8]>) -> GenericHash {
        blake3::hash(data.as_ref()).as_bytes().to_owned().into()
    }

    /// [hash_to_deg] leverages the [hasher] function to hash a given input to a given degree
    pub fn hash_to_deg(data: impl AsRef<[u8]>, deg: Option<usize>) -> GenericHash {
        let hs = hasher(data);
        let mut res: GenericHash = hs;
        for _ in 0..deg.unwrap_or(1) {
            res = hasher(res);
        }
        res
    }

    pub fn hash_serialize<T: Serialize>(data: &T) -> H256 {
        let serialized = bincode::serialize(data).expect("");
        blake3::hash(&serialized).into()
    }

    /// Given a collection of elements, reduce into a single hash by updating the same hasher
    pub fn iter_hasher(data: &Vec<impl AsRef<[u8]>>) -> GenericHash {
        let mut hasher = blake3::Hasher::default();
        for i in data {
            hasher.update(i.as_ref());
        }
        hasher.finalize().as_bytes().to_owned().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generate_random_string;

    #[test]
    fn test_hasher() {
        let a = hasher(generate_random_string(None));
        let b = hasher(generate_random_string(None));
        assert_ne!(&a, &b)
    }

    #[test]
    fn test_iter_hasher() {
        let hashes = |i: usize| {
            std::ops::Range { start: 0, end: i }
                .map(|_| generate_random_string(None))
                .collect::<Vec<String>>()
        };
        let a = iter_hasher(&hashes(10));
        let b = iter_hasher(&hashes(12));
        assert_ne!(&a, &b)
    }
}
