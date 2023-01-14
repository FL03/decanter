/*
    Appellation: decanter <library>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
#[doc(inline)]
#[cfg(feature = "crypto")]
pub use decanter_crypto as crypto;
#[cfg(feature = "derive")]
pub use decanter_derive::*;

pub mod prelude {
    pub use super::*;
    #[cfg(feature = "crypto")]
    pub use super::crypto::*;
}