/*
    Appellation: utils <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use rand::{distributions::Alphanumeric, Rng};

pub fn generate_random_list(n: Option<usize>) -> Vec<u8> {
    (0..n.unwrap_or(32))
        .map(|_| rand::thread_rng().gen::<u8>())
        .collect()
}

/// Simple function wrapper for generating random strings of a given length; defaults to 12
pub fn generate_random_string(length: Option<usize>) -> String {
    let gen = |_i: usize| rand::thread_rng().sample(Alphanumeric) as char;
    (0..length.unwrap_or(12)).map(gen).collect::<String>()
}

pub trait Generative {}

impl Generative for Vec<u8> {}

impl Generative for String {}

pub struct Generator {}
