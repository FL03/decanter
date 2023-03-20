/*
    Appellation: hashes <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use self::{h160::*, h256::*};

pub(crate) mod h160;
pub(crate) mod h256;

use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use strum::{Display, EnumVariantNames};

#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Display,
    EnumVariantNames,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    SmartDefault,
)]
#[strum(serialize_all = "title_case")]
pub enum Hashes {
    #[default]
    H256(H256),
    H160(H160),
}
