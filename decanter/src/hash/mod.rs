/*
    Appellation: hash <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::{hashes::*, iter::*};

mod hashes;
mod iter;

use crate::GenericHash;

/// [Hashable] is a trait that defines a hashable object
pub trait Hashable: ToString {
    fn hash(&self) -> H256 {
        hasher(self.to_string().as_bytes()).into()
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

/// hasher implements a generic hash function wrapper around blake3
pub fn hasher(data: impl AsRef<[u8]>) -> GenericHash {
    blake3::hash(data.as_ref()).as_bytes().to_owned().into()
}
/// Given a collection of elements, reduce into a single hash by updating the same hasher
pub fn iter_hasher(data: &Vec<impl AsRef<[u8]>>) -> GenericHash {
    let mut hasher = blake3::Hasher::default();
    for i in data {
        hasher.update(i.as_ref());
    }
    hasher.finalize().as_bytes().to_owned().into()
}

pub fn hash_to_deg(data: impl AsRef<[u8]>, deg: Option<usize>) -> GenericHash {
    let hs = hasher(data);
    let mut res: GenericHash = hs;
    for _ in 0..deg.unwrap_or(1) {
        res = hasher(res);
    }
    res
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
