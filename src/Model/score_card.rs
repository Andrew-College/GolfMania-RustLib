#[derive(Debug, Default)]
pub struct ScoreCard {
    _name: String,
    _map: String,
    _score: i32
}

impl ScoreCard {
    pub fn new(name: String, map: String, score: i32) -> ScoreCard {
        ScoreCard {
            _name : name,
            _map : map,
            _score : score,
        }
    }

    pub fn name(&self) -> &String {
        &self._name
    }
}

#[cfg(test)]
mod tests {
    use super::ScoreCard;

    #[test]
    fn it_works() {
        let test = ScoreCard::new("testMan".to_string(), "testMap".to_string(), 0);

        assert_eq!(test._name, "testMan");
    }
}