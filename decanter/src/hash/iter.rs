/*
    Appellation: iter <module>
    Contributors: FL03 <jo3mccain@icloud.com>
*/
use super::{Hashable, H256};
use serde::{Deserialize, Serialize};
use std::ops;

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Iter<T>
where
    T: Hashable,
{
    index: usize,
    iter: Vec<T>,
}

impl<T> Iter<T>
where
    T: Hashable,
{
    pub fn new() -> Self {
        Self {
            index: 0,
            iter: Vec::new(),
        }
    }
}

impl<T> std::fmt::Display for Iter<T>
where
    T: Hashable + ToString,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tmp = self.iter.iter().map(|i| i.to_string()).collect::<Vec<_>>();
        write!(f, "{}", serde_json::to_string(&tmp).unwrap())
    }
}

impl<T> Hashable for Iter<T>
where
    T: Hashable + Serialize,
{
    fn hash(&self) -> H256 {
        let mut hasher = blake3::Hasher::new();
        for item in self.iter.iter() {
            let ser = bincode::serialize(item).expect("Failed to serialize item");
            hasher.update(&ser);
        }
        let hash = hasher.finalize();
        hash.into()
    }
}

// impl<T> Hashable for Iter<T>
// where
//     T: Hashable + Serialize,
// {
//     fn hash(&self) -> H256 {
//         let mut hasher = blake3::Hasher::new();
//         for item in self.iter.iter() {
//             let tmp = bincode::serialize(item).expect("Failed to serialize item");
//             hasher.update(&tmp);
//         }
//         let hash = hasher.finalize();
//         hash.into()
//     }
// }

// impl<T> Hasher for Iter<T>
// where
//     T: Hashable + ToString,
// {
//     type Hash = H256;

//     fn hash(data: impl AsRef<[u8]>) -> Self::Hash {
//         blake3::hash(data.as_ref()).into()
//     }

//     fn finalize(self) -> Self::Hash {
//         Hashable::hash(&self)
//     }

//     fn update(&mut self, data: impl AsRef<[u8]>) -> &mut Self {
//         self.iter.push(data);
//         self
//     }
// }

impl<T> ExactSizeIterator for Iter<T>
where
    T: Hashable,
{
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<T> Extend<T> for Iter<T>
where
    T: Hashable,
{
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.iter.extend(iter);
    }
}

impl<T> Iterator for Iter<T>
where
    T: Hashable,
{
    type Item = H256;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.iter.len() {
            let res = self.iter[self.index].hash();
            self.index += 1;
            Some(res)
        } else {
            None
        }
    }
}

impl<T> From<Vec<T>> for Iter<T>
where
    T: Hashable,
{
    fn from(iter: Vec<T>) -> Self {
        Self { index: 0, iter }
    }
}

impl<T> ops::Index<usize> for Iter<T>
where
    T: Hashable,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.iter[index]
    }
}

impl<T> ops::Index<ops::Range<usize>> for Iter<T>
where
    T: Hashable,
{
    type Output = [T];

    fn index(&self, index: ops::Range<usize>) -> &Self::Output {
        &self.iter[index]
    }
}

impl<T> ops::IndexMut<usize> for Iter<T>
where
    T: Hashable,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.iter[index]
    }
}

impl<T> ops::IndexMut<ops::Range<usize>> for Iter<T>
where
    T: Hashable,
{
    fn index_mut(&mut self, index: ops::Range<usize>) -> &mut Self::Output {
        &mut self.iter[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hasher() {
        let data = vec![1, 2, 3, 4, 5];
        let mut iter = Iter::new();
        iter.extend(data.clone());

        for i in data.clone() {
            // Check that each value is hashed correctly
            assert_eq!(iter.next(), Some(H256::new(i.to_string())));
        }
        // Assert that the hash of the iterator produces a single value
        assert_eq!(iter.hash(), crate::hash::hash_iter_ser(&data).into(),);
    }
}
