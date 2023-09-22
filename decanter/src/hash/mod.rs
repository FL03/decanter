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

pub trait Hash {

    fn hash(data: impl AsRef<[u8]>) -> Self;
}

/// [Hashable] is a trait that defines a hashable object
pub trait Hashable: ToString {
    
    fn hash(&self) -> H256 {
        blake3::hash(self.to_string().as_bytes()).into()
    }
}

impl Hashable for char {}

impl Hashable for String {}

impl Hashable for &str {}

impl Hashable for u8 {}

impl Hashable for u16 {}

impl Hashable for u32 {}

impl Hashable for u64 {}

impl Hashable for u128 {}

impl Hashable for usize {}

impl Hashable for i8 {}

impl Hashable for i16 {}

impl Hashable for i32 {}

impl Hashable for i64 {}

impl Hashable for i128 {}

impl Hashable for isize {}

impl Hashable for bool {}

impl Hashable for f32 {}

impl Hashable for f64 {}

pub(crate) mod utils {
    use super::GenericHash;

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
