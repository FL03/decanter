/*
    Appellation: hashing <example>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: A simple example demonstrating the use of the Hashable trait.

        'Hello, world!' is expected to hash to the following value: ede5c0b10f2ec4979c69b52f61e42ff5b413519ce09be0f14d098dcfe5f6f98d
*/
use decanter::hash::{hasher, Hashable, H256};

fn main() {
    let sample = Sample::new("Hello, world!".to_string());
    println!("Hash: {}", sample.hash());
    assert_eq!(sample.hash(), H256::from(hasher("Hello, world!")));
}

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Sample {
    pub value: String,
}

impl Sample {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}

impl Hashable for Sample {
    fn hash(&self) -> H256 {
        hasher(self.to_string()).into()
    }
}

impl std::fmt::Display for Sample {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
