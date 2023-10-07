/*
    Appellation: specs <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub trait Concat {
    fn concat(&mut self, other: &Self) -> Self;
}

impl Concat for blake3::Hash {
    fn concat(&mut self, other: &Self) -> Self {
        let mut res: Vec<u8> = self.as_bytes().to_vec();

        let mut rhs: Vec<u8> = other.as_bytes().to_vec();
        res.append(&mut rhs);
        blake3::hash(&res)
    }
}
