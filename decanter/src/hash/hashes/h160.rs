/*
    Appellation: h160 <module>
    Contributors: FL03 <jo3mccain@icloud.com>
*/
use super::{H160Hash, H256};
use crate::prelude::{Concat, Hashable};

use rand::Rng;
use serde::{Deserialize, Serialize};
use std::ops;

#[derive(Clone, Copy, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct H160(pub H160Hash); // big endian u256

impl H160 {
    pub fn new(data: H160Hash) -> Self {
        Self(data)
    }
    /// Generate a random [H160] hash.
    pub fn generate() -> Self {
        let mut rng = rand::thread_rng();
        let random_bytes: Vec<u8> = (0..20).map(|_| rng.gen()).collect();
        let mut raw_bytes = [0; 20];
        raw_bytes.copy_from_slice(&random_bytes);
        (&raw_bytes).into()
    }
}

impl AsMut<[u8]> for H160 {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl AsRef<[u8]> for H160 {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsMut<[u8; 20]> for H160 {
    fn as_mut(&mut self) -> &mut [u8; 20] {
        &mut self.0
    }
}

impl AsRef<[u8; 20]> for H160 {
    fn as_ref(&self) -> &[u8; 20] {
        &self.0
    }
}

impl ops::Deref for H160 {
    type Target = H160Hash;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ops::DerefMut for H160 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Concat for H160 {
    fn concat(&mut self, other: &H160) -> Self {
        let mut res: Vec<u8> = (*self).into();
        let mut rnode: Vec<u8> = (*other).into();
        res.append(&mut rnode);

        blake3::hash(&res).into()
    }
}

impl Hashable for H160 {
    fn hash(&self) -> H256 {
        (*self).into()
    }
}

impl std::fmt::Debug for H160 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{:>02x}{:>02x}..{:>02x}{:>02x}",
            &self.0[0], &self.0[1], &self.0[18], &self.0[19]
        )
    }
}

impl std::fmt::Display for H160 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let start = if let Some(precision) = f.precision() {
            if precision >= 40 {
                0
            } else {
                20 - precision / 2
            }
        } else {
            0
        };
        for byte_idx in start..20 {
            write!(f, "{:>02x}", &self.0[byte_idx])?;
        }
        Ok(())
    }
}

impl<T> From<&T> for H160
where
    T: AsRef<[u8]>,
{
    fn from(input: &T) -> H160 {
        H160::from(blake3::hash(input.as_ref()))
    }
}

impl From<[u8; 20]> for H160 {
    fn from(input: [u8; 20]) -> H160 {
        H160(input)
    }
}

impl From<H160> for [u8; 20] {
    fn from(input: H160) -> [u8; 20] {
        input.0
    }
}

impl From<Vec<u8>> for H160 {
    fn from(input: Vec<u8>) -> H160 {
        let mut buffer: H160Hash = [0; 20];
        buffer[..].copy_from_slice(&input[0..20]);
        H160(buffer)
    }
}

impl From<H160> for Vec<u8> {
    fn from(input: H160) -> Vec<u8> {
        input.0.to_vec()
    }
}

impl From<blake3::Hash> for H160 {
    fn from(input: blake3::Hash) -> H160 {
        let mut buffer: H160Hash = [0; 20];
        buffer[..].copy_from_slice(&input.as_bytes()[0..20]);
        H160(buffer)
    }
}

impl ops::Index<usize> for H160 {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl ops::Index<ops::Range<usize>> for H160 {
    type Output = [u8];

    fn index(&self, index: ops::Range<usize>) -> &Self::Output {
        &self.0[index]
    }
}

impl ops::IndexMut<usize> for H160 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl ops::IndexMut<ops::Range<usize>> for H160 {
    fn index_mut(&mut self, index: ops::Range<usize>) -> &mut Self::Output {
        &mut self.0[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_h160() {
        let a = H160::generate();
        assert_ne!(a, H160::from(&H256::generate()));
    }

    #[test]
    fn test_concat() {
        let mut a = H160::generate();
        let b = H160::generate();
        let expected: H160 = {
            let mut res: Vec<u8> = a.into();
            let mut rnode: Vec<u8> = b.into();
            res.append(&mut rnode);
            blake3::hash(&res).into()
        };
        assert_eq!(a.concat(&b), expected);
    }
}
