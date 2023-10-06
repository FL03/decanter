/*
    Appellation: specs <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub trait Concat {
    fn concat(&mut self, other: &Self) -> Self;
}
