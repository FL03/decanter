/*
    Appellation: decanter <library>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
#[doc(inline)]
#[cfg(feature = "core")]
pub use decanter_core as core;
#[cfg(feature = "crypto")]
pub use decanter_crypto as crypto;
