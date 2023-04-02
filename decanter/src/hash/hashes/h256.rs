/*
    Appellation: h256 <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::hash::{H256Hash, Hashable, Hasher, H160};
use crate::GenericHash;
use rand::Rng;
use serde::{Deserialize, Serialize};

/// A SHA256 hash.
#[derive(Clone, Copy, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct H256(pub H256Hash);

impl H256 {
    pub fn new(data: impl AsRef<[u8]>) -> Self {
        H256::from(blake3::hash(data.as_ref()))
    }
    /// Concatenates two hashes.
    pub fn concat(&mut self, other: &H256) -> &mut Self {
        let hash = {
            let mut hasher = blake3::Hasher::new();
            hasher.update(self.as_ref());
            hasher.update(other.as_ref());
            hasher.finalize()
        };
        self.0 = hash.into();
        self
    }
    pub fn generate() -> Self {
        let mut rng = rand::thread_rng();
        let random_bytes: Vec<u8> = (0..32).map(|_| rng.gen()).collect();
        let mut raw_bytes = [0; 32];
        raw_bytes.copy_from_slice(&random_bytes);
        (&raw_bytes).into()
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

impl Hashable for H256 {
    fn hash(&self) -> H256 {
        *self
    }
}

impl Hasher for H256 {}

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

impl From<GenericHash> for H256 {
    fn from(data: GenericHash) -> H256 {
        data.as_slice().to_owned().into()
    }
}

impl From<H160> for H256 {
    fn from(input: H160) -> H256 {
        let mut buffer: H256Hash = [0; 32];
        buffer[..].copy_from_slice(&input.0[0..20]);
        buffer.into()
    }
}

impl From<[u8; 32]> for H256 {
    fn from(input: [u8; 32]) -> H256 {
        H256::from_iter(input.to_vec())
    }
}

impl From<Vec<u8>> for H256 {
    fn from(input: Vec<u8>) -> H256 {
        let mut raw_hash: [u8; 32] = [0; 32];
        raw_hash[0..32].copy_from_slice(input.as_ref());
        H256::from_iter(input.to_vec())
    }
}

impl From<blake3::Hash> for H256 {
    fn from(input: blake3::Hash) -> H256 {
        let mut raw_hash: [u8; 32] = [0; 32];
        raw_hash[0..32].copy_from_slice(input.as_bytes());
        H256(raw_hash)
    }
}

impl std::ops::Add<Self> for H256 {
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

impl std::ops::Add<f64> for H256 {
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

impl std::ops::Div<f64> for H256 {
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

impl std::ops::DivAssign<f64> for H256 {
    fn div_assign(&mut self, rhs: f64) {
        for n in 1..9 {
            let results: [u8; 4] = {
                let v = u32::from_be_bytes(self.0[4 * (n - 1)..4 * n].try_into().unwrap());
                let tmp = ((v as f64) / rhs) as u32;
                tmp.to_be_bytes()
            };
            self.0[4 * (n - 1)] = results[0];
            self.0[4 * (n - 1) + 1] = results[1];
            self.0[4 * (n - 1) + 2] = results[2];
            self.0[4 * (n - 1) + 3] = results[3];
        }
    }
}

impl std::ops::Mul<f64> for H256 {
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

impl std::ops::MulAssign<f64> for H256 {
    fn mul_assign(&mut self, rhs: f64) {
        for n in 1..9 {
            let results: [u8; 4] = {
                let v = u32::from_be_bytes(self.0[4 * (n - 1)..4 * n].try_into().unwrap());
                let tmp = ((v as f64) * rhs) as u32;
                tmp.to_be_bytes()
            };
            self.0[4 * (n - 1)] = results[0];
            self.0[4 * (n - 1) + 1] = results[1];
            self.0[4 * (n - 1) + 2] = results[2];
            self.0[4 * (n - 1) + 3] = results[3];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let mut expected: H256 = {
            let mut hasher = blake3::Hasher::new();
            hasher.update(a.clone().as_ref());
            hasher.update(b.clone().as_ref());
            hasher.finalize().into()
        };
        assert_eq!(a.concat(&b), &mut expected);
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
