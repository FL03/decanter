#[cfg(test)]
use decanter_crypto::{hasher, generate_random_string, iter_hasher};

#[test]
fn lib_compiles() {
    let f = |i: usize| i + 1;
    assert_eq!(f(10), 11);
    assert_ne!(f(10), 9)
}


#[test]
fn test_hasher() {
    let a = hasher(&generate_random_string(None));
    let b = hasher(&generate_random_string(None));
    assert_ne!(&a, &b)
}

#[test]
fn test_iter_hasher() {
    let hashes = |i: usize| {
        std::ops::Range { start: 0, end: i }
            .map(|_| generate_random_string(None))
            .collect::<Vec<String>>()
    };
    let a = iter_hasher(&hashes(10));
    let b = iter_hasher(&hashes(12));
    assert_ne!(&a, &b)
}