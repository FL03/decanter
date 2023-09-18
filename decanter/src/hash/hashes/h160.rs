/*
    Appellation: h160 <module>
    Contributors: FL03 <jo3mccain@icloud.com>
*/
use super::{H160Hash, H256};
use crate::hash::Hashable;

use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct H160(pub H160Hash); // big endian u256

impl H160 {
    pub fn new(data: H160Hash) -> Self {
        Self(data)
    }
    /// Concatenates two hashes.
    pub fn concat(&mut self, other: &H160) -> &mut Self {
        let hash = {
            let mut hasher = blake3::Hasher::new();
            hasher.update(self.as_ref());
            hasher.update(other.as_ref());
            hasher.finalize()
        };
        *self = hash.into();
        self
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

impl From<blake3::Hash> for H160 {
    fn from(input: blake3::Hash) -> H160 {
        let mut buffer: H160Hash = [0; 20];
        buffer[..].copy_from_slice(&input.as_bytes()[0..20]);
        H160(buffer)
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
        let mut expected: H160 = {
            let mut hasher = blake3::Hasher::new();
            hasher.update(a.clone().as_ref());
            hasher.update(b.clone().as_ref());
            hasher.finalize().into()
        };
        assert_eq!(a.concat(&b), &mut expected);
    }
}
