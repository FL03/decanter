/*
    Appellation: derive <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[cfg(test)]
#[cfg(feature = "derive")]
mod tests {
    use decanter::prelude::{hash_serialize, *};
    use serde::{Deserialize, Serialize};

    #[derive(
        Clone,
        Copy,
        Debug,
        Default,
        Deserialize,
        Eq,
        Hash,
        Hashable,
        Ord,
        PartialEq,
        PartialOrd,
        Serialize,
    )]
    #[decanter(string)]
    pub struct Sample(i64);

    impl Sample {
        pub fn new(data: i64) -> Self {
            Self(data)
        }
    }

    impl std::fmt::Display for Sample {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    #[derive(
        Clone,
        Copy,
        Debug,
        Default,
        Deserialize,
        Eq,
        Hash,
        Hashable,
        Ord,
        PartialEq,
        PartialOrd,
        Serialize,
    )]
    #[decanter(serde)]
    pub struct TestStruct(i64);

    impl TestStruct {
        pub fn new(data: i64) -> Self {
            Self(data)
        }
    }

    impl std::fmt::Display for TestStruct {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    #[test]
    fn test_derive_hashable() {
        let data: i64 = 0;
        let a = Sample::new(data);
        let b = TestStruct::new(data);
        let str_hash: H256 = hasher(data.to_string()).into();
        assert_eq!(a.hash(), str_hash);
        assert_eq!(b.hash(), hash_serialize(&data));
        assert_ne!(a.hash(), b.hash());
    }
}
