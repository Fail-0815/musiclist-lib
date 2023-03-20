use serde::{Deserialize, Serialize};

#[derive(PartialEq, Deserialize, Serialize, Clone, Debug)]
pub struct MusicWish {
    pub id: i32,
    pub title: String,
    pub artist: String,
    pub comment: String,
    pub voted: bool,
    pub score: usize,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LoginResult {
    pub status: bool,
    pub message: String,
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
