/*
    Appellation: hasher <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::H256;

pub struct Iter<T>
where
    T: ToString,
{
    index: usize,
    iter: Vec<T>,
}

impl<T> Iter<T>
where
    T: ToString,
{
    pub fn new() -> Self {
        Self {
            index: 0,
            iter: Vec::new(),
        }
    }
}
