/*
    Appellation: secret_key <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SecretKey {
    id: String,
    pub(crate) key: Vec<u8>,
    ts: i64,
}
