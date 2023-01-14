/*
    Appellation: derive <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:  ... Summary ...
*/
#[cfg(feature = "derive")]
#[cfg(test)]
use decanter::prelude::{Hash, Hashable};
use scsys::prelude::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Default, Hash, Deserialize, Serialize)]
pub struct TestStruct {
    timestamp: Timestamp,
}

impl std::fmt::Display for TestStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.timestamp)
    }
}


#[derive(Default)]
struct Pancakes;

#[test]
fn test_hashable_derive() {
    let a = TestStruct::default();
    let _hash = a.hash();
    let _string = a.to_string();
    assert!(true)
}