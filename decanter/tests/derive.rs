/*
    Appellation: derive <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[cfg(test)]
#[cfg(feature = "derive")]
mod tests {
    use decanter::prelude::*;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Default, Deserialize, Hashable, Serialize, Shash)]
    pub struct TestStruct {
        id: i64,
    }

    impl TestStruct {
        pub fn new(id: i64) -> Self {
            Self { id }
        }
    }

    impl std::fmt::Display for TestStruct {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.id)
        }
    }

    #[test]
    fn test_derive_hashable() {
        let data: i64 = 0;
        let a = TestStruct::new(data);
        let exp: H256 = hasher(data.to_string()).into();
        assert_eq!(Hashable::hash(&a), exp);
        assert_ne!(Hashable::hash(&a), Shash::hash(&a));
    }

    #[test]
    fn test_derive_shash() {
        let data: i64 = 0;
        let a = TestStruct::new(data);
        let exp: H256 = hash_serialize(&data);
        assert_eq!(Shash::hash(&a), exp);
    }
}
