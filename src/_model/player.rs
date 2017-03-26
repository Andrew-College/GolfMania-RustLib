pub const ANON: &'static str = "The great unknown; anon";

pub struct Player{
    _name: &'static str
}

impl Player {
    pub fn new(name: Option<&'static str>) -> Player {
        Player {
            _name: match name {
                Some(actual_name) => actual_name,
                None => ANON,
            },
        }
    }
}