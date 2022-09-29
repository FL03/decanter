/*
    Appellation: construct <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum SleeveContext {
    #[default]
    ED25519,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Sleeve {
    pub context: SleeveContext,
}
