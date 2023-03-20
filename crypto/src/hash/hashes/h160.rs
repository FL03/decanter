/*
    Appellation: h160 <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::H256;
use crate::{H160Hash, Hashable};

use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct H160(pub H160Hash); // big endian u256

impl H160 {
    pub fn new(data: H160Hash) -> Self {
        Self(data)
    }
    pub fn generate() -> Self {
        let mut rng = rand::thread_rng();
        let random_bytes: Vec<u8> = (0..20).map(|_| rng.gen()).collect();
        let mut raw_bytes = [0; 20];
        raw_bytes.copy_from_slice(&random_bytes);
        (&raw_bytes).into()
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

impl From<&H160Hash> for H160 {
    fn from(input: &H160Hash) -> H160 {
        let mut buffer: H160Hash = [0; 20];
        buffer[..].copy_from_slice(input);
        H160(buffer)
    }
}

impl From<H160Hash> for H160 {
    fn from(input: H160Hash) -> H160 {
        H160(input)
    }
}

impl From<&H256> for H160 {
    fn from(input: &H256) -> H160 {
        let mut buffer: H160Hash = [0; 20];
        buffer[..].copy_from_slice(&input.0[0..20]);
        buffer.into()
    }
}

impl From<H256> for H160 {
    fn from(input: H256) -> H160 {
        let mut buffer: H160Hash = [0; 20];
        buffer[..].copy_from_slice(&input.0[0..20]);
        buffer.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::H256;

    #[test]
    fn test_h160_random() {
        let a = H160::generate();
        let b = H160::from(H256::generate());
        assert_ne!(a, b)
    }
}
