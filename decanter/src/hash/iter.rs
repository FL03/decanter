/*
    Appellation: iter <module>
    Contributors: FL03 <jo3mccain@icloud.com>
*/
use super::{Hashable, Hasher, H256};
use serde::{Deserialize, Serialize};

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
    T: Hashable,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tmp = {
            let mut s = Vec::new();
            for item in self.iter.iter() {
                s.push(item.to_string());
            }
            s
        };
        write!(f, "{}", serde_json::to_string(&tmp).unwrap())
    }
}

impl<T> Hashable for Iter<T>
where
    T: Hashable,
{
    fn hash(&self) -> H256 {
        let mut hasher = blake3::Hasher::new();
        for item in self.iter.iter() {
            hasher.update(item.to_string().as_bytes());
        }
        let hash = hasher.finalize();
        hash.into()
    }
}

impl<T> Hasher for Iter<T>
where
    T: Hashable,
{
    type Hash = H256;

    fn hash(data: impl AsRef<[u8]>) -> Self::Hash {
        blake3::hash(data.as_ref()).into()
    }
}

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

impl<T> std::ops::Index<usize> for Iter<T>
where
    T: Hashable,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.iter[index]
    }
}

impl<T> std::ops::Index<std::ops::Range<usize>> for Iter<T>
where
    T: Hashable,
{
    type Output = [T];

    fn index(&self, index: std::ops::Range<usize>) -> &Self::Output {
        &self.iter[index]
    }
}

impl<T> std::ops::IndexMut<usize> for Iter<T>
where
    T: Hashable,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.iter[index]
    }
}

impl<T> std::ops::IndexMut<std::ops::Range<usize>> for Iter<T>
where
    T: Hashable,
{
    fn index_mut(&mut self, index: std::ops::Range<usize>) -> &mut Self::Output {
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
        assert_eq!(
            iter.hash(),
            crate::hash::iter_hasher(
                &data
                    .into_iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<String>>()
            )
            .into(),
        );
    }
}
