/*
    Appellation: derive <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:  ... Summary ...
*/
#[cfg(test)]
#[cfg(feature = "derive")]
mod tests {
    use decanter::prelude::{hasher, Hashable, H256};

    #[derive(Clone, Debug, Default, Hashable)]
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
    fn test_hashable_derive() {
        let a = TestStruct::new(0_i64);
        assert_eq!(a.hash(), hasher(0_i64.to_string()).into());
    }
}
