// hashes.rs
#![feature(test)]

extern crate test;

use decanter::hash::H256;
use hex_literal::hex;
use std::collections::HashMap;
use test::Bencher;

lazy_static::lazy_static!(
    static ref SAMPLE_ENTRIES: Vec<String> = ["a", "b", "c", "d", "e"].iter().map(|i| i.to_string()).collect();
    static ref SAMPLE_DATA: HashMap<String, H256> = {
        let d = ["a", "b", "c", "d", "e"];
        let mut m = HashMap::new();
        for (i, key) in d.iter().enumerate() {
            m.insert(key.to_string(), SAMPLE_HASHES[i]);
        }
        m
    };
    static ref SAMPLE_HASHES: Vec<H256> = vec![
            (hex!("17762fddd969a453925d65717ac3eea21320b66b54342fde15128d6caf21215f")).into(),
            (hex!("10e5cf3d3c8a4f9f3468c8cc58eea84892a22fdadbc1acb22410190044c1d553")).into(),
            (hex!("ea7aa1fc9efdbe106dbb70369a75e9671fa29d52bd55536711bf197477b8f021")).into(),
            (hex!("d5ede538f628f687e5e0422c7755b503653de2dcd7053ca8791afa5d4787d843")).into(),
            (hex!("27bb492e108bf5e9c724176d7ae75d4cedc422fe4065020bd6140c3fcad3a9e7")).into(),
        ];
);

#[bench]
fn iterative_hash(b: &mut Bencher) {
    b.iter(|| {
        for i in SAMPLE_ENTRIES.clone() {
            assert_eq!(H256::new(i.clone()), *SAMPLE_DATA.get(&i).unwrap());
        }
    })
}
