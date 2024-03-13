/*
    Appellation: kinds <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIs, EnumIter, EnumString, VariantNames};

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Deserialize,
    Display,
    EnumCount,
    EnumIs,
    EnumIter,
    EnumString,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    VariantNames,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum HashSize {
    H32 = 4,
    H64 = 8,
    H128 = 16,
    H160 = 20,
    #[default]
    H256 = 32,
    H320 = 40,
    H512 = 64,
}

impl From<HashSize> for usize {
    fn from(size: HashSize) -> usize {
        size as usize
    }
}
