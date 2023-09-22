/*
    Appellation: hash <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{Concat, Hasher};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::ops;

/// A SHA256 hash.
#[derive(Clone, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Hash {
    digest: Vec<u8>,
    size: usize,
}

impl Hash {
    pub fn new(data: impl AsRef<[u8]>, size: Option<usize>) -> Self {
        Self {
            digest: blake3::hash(data.as_ref()).as_bytes().to_vec(),
            size: size.unwrap_or(32),
        }
    }
    pub fn generate() -> Self {
        let mut rng = rand::thread_rng();
        let random_bytes: Vec<u8> = (0..32).map(|_| rng.gen()).collect();
        let mut raw_bytes = [0; 32];
        raw_bytes.copy_from_slice(&random_bytes);
        (&raw_bytes).into()
    }
}

impl AsMut<[u8]> for Hash {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.digest
    }
}

impl AsRef<[u8]> for Hash {
    fn as_ref(&self) -> &[u8] {
        &self.digest
    }
}

impl Concat for Hash {
    fn concat(&mut self, other: &Self) -> Self {
        let mut res: Vec<u8> = self.clone().into();
        let mut rnode: Vec<u8> = other.clone().into();
        res.append(&mut rnode);

        blake3::hash(&res).as_bytes().into()
    }
}

// impl Hashable for Hash {
//     fn hash(&self) -> super::H256 {
//         *self
//     }
// }

impl Hasher for Hash {
    type Hash = Self;
}

impl Ord for Hash {
    fn cmp(&self, other: &Hash) -> std::cmp::Ordering {
        let (half, full) = (self.size / 2, self.size);
        let self_higher = u128::from_be_bytes(self[0..half].try_into().unwrap());
        let self_lower = u128::from_be_bytes(self[half..full].try_into().unwrap());
        let other_higher = u128::from_be_bytes(other[0..half].try_into().unwrap());
        let other_lower = u128::from_be_bytes(other[half..full].try_into().unwrap());
        let higher = self_higher.cmp(&other_higher);
        match higher {
            std::cmp::Ordering::Equal => self_lower.cmp(&other_lower),
            _ => higher,
        }
    }
}

impl PartialOrd for Hash {
    fn partial_cmp(&self, other: &Hash) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::fmt::Debug for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{:>02x}{:>02x}..{:>02x}{:>02x}",
            &self[0],
            &self[1],
            &self[self.size - 2],
            &self[self.size - 1]
        )
    }
}

impl std::fmt::Display for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let start = if let Some(precision) = f.precision() {
            if precision >= self.size * 2 {
                0
            } else {
                self.size - precision / 2
            }
        } else {
            0
        };
        for byte_idx in start..self.size {
            write!(f, "{:>02x}", &self[byte_idx])?;
        }
        Ok(())
    }
}

impl<T> From<&T> for Hash
where
    T: AsRef<[u8]>,
{
    fn from(bytes: &T) -> Self {
        let digest = bytes.as_ref().to_vec();
        let size = digest.len();
        Self { digest, size }
    }
}

impl From<Vec<u8>> for Hash {
    fn from(bytes: Vec<u8>) -> Self {
        let size = bytes.len();
        Self {
            digest: bytes,
            size,
        }
    }
}

impl From<Hash> for Vec<u8> {
    fn from(hash: Hash) -> Self {
        hash.digest
    }
}

impl IntoIterator for Hash {
    type Item = u8;
    type IntoIter = std::vec::IntoIter<u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.digest.into_iter()
    }
}

impl ops::Index<usize> for Hash {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.digest[index]
    }
}

impl ops::IndexMut<usize> for Hash {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.digest[index]
    }
}

impl ops::Index<ops::Range<usize>> for Hash {
    type Output = [u8];

    fn index(&self, index: ops::Range<usize>) -> &Self::Output {
        &self.digest[index]
    }
}

impl ops::IndexMut<ops::Range<usize>> for Hash {
    fn index_mut(&mut self, index: ops::Range<usize>) -> &mut Self::Output {
        &mut self.digest[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hash::H256;

    #[test]
    fn test_hash() {
        assert_ne!(Hash::generate(), Hash::generate());

        let data = [b"a", b"b", b"c", b"d"];
        let exp = {
            let mut res = Vec::<H256>::new();
            for d in data {
                res.push(blake3::hash(d).into());
            }
            res
        };
        assert_eq!(H256::new(data[0]), exp[0]);
    }
}
