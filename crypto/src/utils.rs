/*
    Appellation: utils <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use ring::{pkcs8, rand::SystemRandom, signature::Ed25519KeyPair};

/// Generate a random key pair.
pub fn random_keypair() -> Ed25519KeyPair {
    Ed25519KeyPair::from_pkcs8(generate_random_pkcs8().as_ref()).unwrap()
}
/// Generate a random PKCS8 document.
pub fn generate_random_pkcs8() -> pkcs8::Document {
    Ed25519KeyPair::generate_pkcs8(&SystemRandom::new()).unwrap()
}
