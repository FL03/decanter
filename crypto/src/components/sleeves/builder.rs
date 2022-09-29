/*
    Appellation: builder <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
use super::Sleeve;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct SleeveBuilder {
    pub sleeve: Sleeve,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let f = |x: usize, y: usize| x + y;
        let actual = f(4, 4);
        let expected: usize = 8;
        assert_eq!(actual, expected)
    }
}
