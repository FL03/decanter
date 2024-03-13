/*
    Appellation: specs <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::gen::*;

pub(crate) mod gen;

pub trait Concat {
    fn concat(&mut self, other: &Self) -> Self;
}

impl Concat for blake3::Hash {
    fn concat(&mut self, other: &Self) -> Self {
        let mut lhs: Vec<u8> = self.as_bytes().to_vec();
        let mut rhs: Vec<u8> = other.as_bytes().to_vec();

        lhs.append(&mut rhs);

        blake3::hash(&lhs)
    }
}
