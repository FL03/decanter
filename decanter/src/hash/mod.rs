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
    fn hash(&self) -> H256;
}

///
pub trait HashableExt: Hashable {
    fn hasher(&self, deg: Option<usize>) -> H256 {
        let s: H256 = hasher(&self.to_string()).into();

        let mut res: H256 = s;
        for _ in 0..deg.unwrap_or(1) {
            res = hasher(&res.clone()).into()
        }
        res
    }
}

/// hasher implements a generic hash function wrapper around blake3
pub fn hasher(data: &impl ToString) -> GenericHash {
    blake3::hash(data.to_string().as_bytes())
        .as_bytes()
        .to_owned()
        .into()
}
/// Given a collection of elements, reduce into a single hash by updating the same hasher
pub fn iter_hasher<T: ToString>(data: &Vec<T>) -> GenericHash {
    let mut hasher = blake3::Hasher::default();
    for i in data {
        hasher.update(i.to_string().as_bytes());
    }
    hasher.finalize().as_bytes().to_owned().into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generate_random_string;

    #[test]
    fn test_hasher() {
        let a = hasher(&generate_random_string(None));
        let b = hasher(&generate_random_string(None));
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
