/*
    Appellation: h256 <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{H256Hash, H160};
use crate::hash::{GenericHash, Hashable, Hasher};
use crate::Concat;

use rand::Rng;
use serde::{Deserialize, Serialize};
use std::ops;

/// A SHA256 hash.
#[derive(Clone, Copy, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct H256(pub H256Hash);

impl H256 {
    pub fn new(data: impl AsRef<[u8]>) -> Self {
        H256::from(blake3::hash(data.as_ref()))
    }
    pub fn generate() -> Self {
        let mut rng = rand::thread_rng();
        let random_bytes: Vec<u8> = (0..32).map(|_| rng.gen()).collect();
        let mut raw_bytes = [0; 32];
        raw_bytes.copy_from_slice(&random_bytes);
        (&raw_bytes).into()
    }

    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl AsMut<[u8]> for H256 {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl AsRef<[u8]> for H256 {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Concat for H256 {
    fn concat(&mut self, other: &Self) -> Self {
        let mut res: Vec<u8> = (*self).into();
        let mut rnode: Vec<u8> = (*other).into();
        res.append(&mut rnode);

        blake3::hash(&res).into()
    }
}

impl Hashable for H256 {
    fn hash(&self) -> H256 {
        *self
    }
}

impl Hasher for H256 {
    type Hash = Self;

    fn hash(data: impl AsRef<[u8]>) -> Self::Hash {
        blake3::hash(data.as_ref()).into()
    }
}

impl Ord for H256 {
    fn cmp(&self, other: &H256) -> std::cmp::Ordering {
        let self_higher = u128::from_be_bytes(self.0[0..16].try_into().unwrap());
        let self_lower = u128::from_be_bytes(self.0[16..32].try_into().unwrap());
        let other_higher = u128::from_be_bytes(other.0[0..16].try_into().unwrap());
        let other_lower = u128::from_be_bytes(other.0[16..32].try_into().unwrap());
        let higher = self_higher.cmp(&other_higher);
        match higher {
            std::cmp::Ordering::Equal => self_lower.cmp(&other_lower),
            _ => higher,
        }
    }
}

impl PartialOrd for H256 {
    fn partial_cmp(&self, other: &H256) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::fmt::Debug for H256 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{:>02x}{:>02x}..{:>02x}{:>02x}",
            &self.0[0], &self.0[1], &self.0[30], &self.0[31]
        )
    }
}

impl std::fmt::Display for H256 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let start = if let Some(precision) = f.precision() {
            if precision >= 64 {
                0
            } else {
                32 - precision / 2
            }
        } else {
            0
        };
        for byte_idx in start..32 {
            write!(f, "{:>02x}", &self.0[byte_idx])?;
        }
        Ok(())
    }
}

impl FromIterator<u8> for H256 {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        let mut buffer: [u8; 32] = [0; 32];
        buffer[..].copy_from_slice(&iter.into_iter().collect::<Vec<_>>()[0..32]);
        H256(buffer)
    }
}

impl IntoIterator for H256 {
    type Item = u8;
    type IntoIter = std::array::IntoIter<u8, 32>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T> From<&T> for H256
where
    T: AsRef<[u8]>,
{
    fn from(data: &T) -> H256 {
        let hash = {
            let mut hasher = blake3::Hasher::new();
            hasher.update(data.as_ref());
            hasher.finalize()
        };
        let mut buffer: [u8; 32] = [0; 32];
        buffer[..].copy_from_slice(hash.as_bytes());
        H256(buffer)
    }
}

impl From<[u8; 32]> for H256 {
    fn from(input: [u8; 32]) -> H256 {
        H256::from_iter(input.to_vec())
    }
}

impl From<H256> for [u8; 32] {
    fn from(input: H256) -> [u8; 32] {
        input.0
    }
}

impl From<Vec<u8>> for H256 {
    fn from(input: Vec<u8>) -> H256 {
        let mut raw_hash: [u8; 32] = [0; 32];
        raw_hash[0..32].copy_from_slice(input.as_ref());
        H256::from_iter(input.to_vec())
    }
}

impl From<H256> for Vec<u8> {
    fn from(input: H256) -> Vec<u8> {
        input.0.to_vec()
    }
}

impl From<blake3::Hash> for H256 {
    fn from(input: blake3::Hash) -> H256 {
        let mut raw_hash: [u8; 32] = [0; 32];
        raw_hash[0..32].copy_from_slice(input.as_bytes());
        H256(raw_hash)
    }
}

impl From<H256> for blake3::Hash {
    fn from(input: H256) -> blake3::Hash {
        blake3::Hash::from(input.0)
    }
}

impl From<GenericHash> for H256 {
    fn from(data: GenericHash) -> H256 {
        data.as_slice().to_owned().into()
    }
}

impl From<H256> for GenericHash {
    fn from(input: H256) -> GenericHash {
        GenericHash::from(input.0)
    }
}

impl From<H160> for H256 {
    fn from(input: H160) -> H256 {
        let mut buffer: H256Hash = [0; 32];
        buffer[..].copy_from_slice(&input.0[0..20]);
        buffer.into()
    }
}

impl From<H256> for H160 {
    fn from(input: H256) -> H160 {
        let mut buffer: super::H160Hash = [0; 20];
        buffer[..].copy_from_slice(&input.0[0..20]);
        buffer.into()
    }
}

impl ops::Add<Self> for H256 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let mut result_bytes = [0; 32];
        for n in 1..9 {
            let results: [u8; 4] = {
                let val = u32::from_be_bytes(self.0[4 * (n - 1)..4 * n].try_into().unwrap());
                let rhs = u32::from_be_bytes(rhs.0[4 * (n - 1)..4 * n].try_into().unwrap());
                let tmp = ((val as u64) + (rhs as u64)) as u32;
                tmp.to_be_bytes()
            };
            result_bytes[4 * (n - 1)] = results[0];
            result_bytes[4 * (n - 1) + 1] = results[1];
            result_bytes[4 * (n - 1) + 2] = results[2];
            result_bytes[4 * (n - 1) + 3] = results[3];
        }
        Self(result_bytes)
    }
}

impl ops::AddAssign<Self> for H256 {
    fn add_assign(&mut self, rhs: Self) {
        for n in 1..9 {
            let results: [u8; 4] = {
                let val = u32::from_be_bytes(self.0[4 * (n - 1)..4 * n].try_into().unwrap());
                let rhs = u32::from_be_bytes(rhs.0[4 * (n - 1)..4 * n].try_into().unwrap());
                let tmp = ((val as u64) + (rhs as u64)) as u32;
                tmp.to_be_bytes()
            };
            self[4 * (n - 1)] = results[0];
            self[4 * (n - 1) + 1] = results[1];
            self[4 * (n - 1) + 2] = results[2];
            self[4 * (n - 1) + 3] = results[3];
        }
    }
}

impl ops::Add<f64> for H256 {
    type Output = Self;

    fn add(self, rhs: f64) -> Self {
        let mut result_bytes = [0; 32];
        for n in 1..9 {
            let results: [u8; 4] = {
                let v = u32::from_be_bytes(self.0[4 * (n - 1)..4 * n].try_into().unwrap());
                let tmp = ((v as f64) + rhs) as u32;
                tmp.to_be_bytes()
            };
            result_bytes[4 * (n - 1)] = results[0];
            result_bytes[4 * (n - 1) + 1] = results[1];
            result_bytes[4 * (n - 1) + 2] = results[2];
            result_bytes[4 * (n - 1) + 3] = results[3];
        }
        Self(result_bytes)
    }
}

impl ops::AddAssign<f64> for H256 {
    fn add_assign(&mut self, rhs: f64) {
        for n in 1..9 {
            let results: [u8; 4] = {
                let v = u32::from_be_bytes(self.0[4 * (n - 1)..4 * n].try_into().unwrap());
                let tmp = ((v as f64) + rhs) as u32;
                tmp.to_be_bytes()
            };
            self[4 * (n - 1)] = results[0];
            self[4 * (n - 1) + 1] = results[1];
            self[4 * (n - 1) + 2] = results[2];
            self[4 * (n - 1) + 3] = results[3];
        }
    }
}

impl ops::Div<f64> for H256 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        let mut result_bytes = [0; 32];
        for n in 1..9 {
            let results: [u8; 4] = {
                let v = u32::from_be_bytes(self.0[4 * (n - 1)..4 * n].try_into().unwrap());
                let tmp = ((v as f64) / rhs) as u32;
                tmp.to_be_bytes()
            };
            result_bytes[4 * (n - 1)] = results[0];
            result_bytes[4 * (n - 1) + 1] = results[1];
            result_bytes[4 * (n - 1) + 2] = results[2];
            result_bytes[4 * (n - 1) + 3] = results[3];
        }
        Self(result_bytes)
    }
}

impl ops::DivAssign<f64> for H256 {
    fn div_assign(&mut self, rhs: f64) {
        for n in 1..9 {
            let results: [u8; 4] = {
                let v = u32::from_be_bytes(self.0[4 * (n - 1)..4 * n].try_into().unwrap());
                let tmp = ((v as f64) / rhs) as u32;
                tmp.to_be_bytes()
            };
            self[4 * (n - 1)] = results[0];
            self[4 * (n - 1) + 1] = results[1];
            self[4 * (n - 1) + 2] = results[2];
            self[4 * (n - 1) + 3] = results[3];
        }
    }
}

impl ops::Mul<f64> for H256 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        let mut result_bytes = [0; 32];
        for n in 1..9 {
            let results: [u8; 4] = {
                let v = u32::from_be_bytes(self.0[4 * (n - 1)..4 * n].try_into().unwrap());
                let tmp = ((v as f64) * rhs) as u32;
                tmp.to_be_bytes()
            };
            result_bytes[4 * (n - 1)] = results[0];
            result_bytes[4 * (n - 1) + 1] = results[1];
            result_bytes[4 * (n - 1) + 2] = results[2];
            result_bytes[4 * (n - 1) + 3] = results[3];
        }
        Self(result_bytes)
    }
}

impl ops::MulAssign<f64> for H256 {
    fn mul_assign(&mut self, rhs: f64) {
        for n in 1..9 {
            let results: [u8; 4] = {
                let v = u32::from_be_bytes(self.0[4 * (n - 1)..4 * n].try_into().unwrap());
                let tmp = ((v as f64) * rhs) as u32;
                tmp.to_be_bytes()
            };
            self[4 * (n - 1)] = results[0];
            self[4 * (n - 1) + 1] = results[1];
            self[4 * (n - 1) + 2] = results[2];
            self[4 * (n - 1) + 3] = results[3];
        }
    }
}

impl ops::Index<usize> for H256 {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl ops::IndexMut<usize> for H256 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl ops::Index<ops::Range<usize>> for H256 {
    type Output = [u8];

    fn index(&self, index: ops::Range<usize>) -> &Self::Output {
        &self.0[index]
    }
}

impl ops::IndexMut<ops::Range<usize>> for H256 {
    fn index_mut(&mut self, index: ops::Range<usize>) -> &mut Self::Output {
        &mut self.0[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hash::concat_b3;

    #[test]
    fn test_h256() {
        let a = H256::generate();
        assert_ne!(a, H256::generate());
        assert_eq!(a, H256::from_iter(a.as_ref().to_vec()));
    }

    #[test]
    fn test_concat() {
        let mut a = H256::generate();
        let b = H256::generate();
        let expected: H256 = concat_b3(a.into(), Some(b.into())).into();
        assert_eq!(a.concat(&b), expected);
    }

    #[test]
    fn test_addition() {
        let a = H256::generate();
        let b = H256::generate();
        assert_ne!(a, a + b);
        assert_eq!(a, a + 0f64);
    }

    #[test]
    fn test_division() {
        let a = H256::generate();
        assert_eq!(a, a / 1f64);
        assert_ne!(a, a / 0f64);
        assert_ne!(a, a / 10f64)
    }

    #[test]
    fn test_multiplication() {
        let a = H256::generate();
        let mut b = a;
        assert_eq!(a, a * 1f64);
        b *= 0f64;
        assert_ne!(a, b);
        assert_eq!(a * 0f64, b);
        assert_ne!(a, a * 10f64)
    }
}
