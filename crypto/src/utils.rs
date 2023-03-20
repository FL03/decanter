/*
    Appellation: utils <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::GenericHash;
use rand::{distributions::Alphanumeric, Rng};
use std::ops::Range;

/// Simple function wrapper for generating random strings of a given length; defaults to 12
pub fn generate_random_string(length: Option<usize>) -> String {
    Range {
        start: 0,
        end: length.unwrap_or(12),
    }
    .map(|_| rand::thread_rng().sample(Alphanumeric) as char)
    .collect::<String>()
}

/// hasher implements a generic hash function wrapper around blake3
pub fn hasher<T: ToString>(data: &T) -> GenericHash {
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
