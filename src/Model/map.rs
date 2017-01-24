// const valid_characters: &'static str = "[H|_,*#]";
const ANGLED: &'static str = include_str!("Maps/Angled.txt");
const DANGER: &'static str = include_str!("Maps/Danger.txt");
const REVERSE: &'static str = include_str!("Maps/Reverse.txt");
const SPIRAL: &'static str = include_str!("Maps/Spiral.txt");
const TUTORIAL: &'static str = include_str!("Maps/Tutorial.txt");
const WIGGLER: &'static str = include_str!("Maps/Wiggler.txt");

#[derive(Debug)]
pub enum MapName {
    Angled,
    Danger,
    Reverse,
    Spiral,
    Tutorial,
    Wiggler,
}

#[derive(Debug)]
pub struct Cell {
    _foreground: char,
    _background: char,
}

impl Cell {
    pub fn fore(&self) -> &char {
        &self._foreground
    }

    pub fn back(&self) -> &char {
        &self._background
    }

    pub fn from_string(input: &str) -> Vec<Cell> {
            input
                .chars()
                .map(
                    |ch| match ch {
                        'H' =>
                            Cell {
                                _foreground: ' ',
                                _background: 'H'
                            },
                        '|' =>
                            Cell {
                                _foreground: '|',
                                _background: '|'
                            },
                        '_' =>
                            Cell {
                                _foreground: '_',
                                _background: '_'
                            },
                        '*' =>
                            Cell {
                                _foreground: '*',
                                _background: ','
                            },
                        '#' =>
                            Cell {
                                _foreground: ' ',
                                _background: '#'
                            },
                        _ =>
                            Cell {
                                _foreground: ' ',
                                _background: ',',
                            },
                    }
                )
                .collect()
        
    }
}

#[derive(Debug)]
pub struct Map {
    _name: String,
    _board: Vec<Vec<Cell>>
}

impl Map {
    pub fn name(&self) -> &String {
        &self._name
    }

    pub fn board(&self) -> &Vec<Vec<Cell>> {
        &self._board
    }

    pub fn from_named(map_name : Option<MapName>) -> Map {
        match map_name {
            Some(map_val) => {
                Map::load_map(map_val)
            },
            None => Map::load_map(MapName::Tutorial),
        }
    }

    fn load_map(selected_map: MapName) -> Map{
        match selected_map {
            MapName::Angled => Map{
                _name: "Angled".to_string(),
                _board: ANGLED.compile_string()
            },
            MapName::Danger => Map{
                _name: "Danger".to_string(),
                _board: DANGER.compile_string()
            },
            MapName::Reverse => Map{
                _name: "Reverse".to_string(),
                _board: REVERSE.compile_string()
            },
            MapName::Spiral => Map{
                _name: "Spiral".to_string(),
                _board: SPIRAL.compile_string()
            },
            MapName::Wiggler => Map{
                _name: "Wiggler".to_string(),
                _board: WIGGLER.compile_string()
            },
            _ => Map{
                _name: "Tutorial".to_string(),
                _board: TUTORIAL.compile_string()
            },
        }
    }
}

trait MapCompiler {
     fn compile_string<'a>(&self) -> Vec<Vec<Cell>>;
}

impl MapCompiler for str {
     fn compile_string<'a>(&self) -> Vec<Vec<Cell>> {
        self
            .lines()
            .map( |line| Cell::from_string(line) )
            .collect()
     }
}

#[cfg(test)]
mod tests {
    use super::{Map, Cell};

    #[test]
    fn first_background_char_of_tutorial() {
        let test = Map::from_named(None);

        assert_eq!(
            *test
                .board()
                    .first()
                        .unwrap()
                    .first()
                        .unwrap()
                .back(), 
            'H'
        );
    }

    #[test]
    fn invalid_characters_fallback() {
        let test = cell::from_string("memes");

        for test_cell in test {
            assert_eq!(*test_cell.back(), ',');
        }
    }
}