/*
    Appellation: utils <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
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
