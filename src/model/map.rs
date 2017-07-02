// const valid_characters: &'static str = "[H|_,*#]";
pub const ANGLED: &'static str = include_str!("Maps/Angled.txt");
pub const DANGER: &'static str = include_str!("Maps/Danger.txt");
pub const REVERSE: &'static str = include_str!("Maps/Reverse.txt");
pub const SPIRAL: &'static str = include_str!("Maps/Spiral.txt");
pub const TUTORIAL: &'static str = include_str!("Maps/Tutorial.txt");
pub const WIGGLER: &'static str = include_str!("Maps/Wiggler.txt");

#[derive(Debug)]
pub enum MapName {
    Angled,
    Danger,
    Reverse,
    Spiral,
    Tutorial,
    Wiggler,
}

#[derive(Builder, Clone, Copy, Debug, Eq, PartialEq)]
pub struct Cell {
    foreground: char,
    background: char,
}

impl CellBuilder {
    pub fn build_string(input: &str) -> Result<Vec<Cell>, String> {
        // Run through the string, building up the Map
        input
            .chars()
            .map(
                |ch| match ch {
                    'H' =>
                        Ok(
                            Cell {
                                foreground: ' ',
                                background: 'H'
                            }
                        ),
                    '|' =>
                        Ok(
                            Cell {
                                foreground: ' ',
                                background: '|'
                            }
                        ),
                    '_' =>
                        Ok (
                            Cell {
                                foreground: ' ',
                                background: '_'
                            }
                        ),
                    '\\' =>
                        Ok (
                            Cell {
                                foreground: ' ',
                                background: '\\'
                            }
                        ),
                    '*' =>
                        Ok (
                            Cell {
                                foreground: '*',
                                background: ','
                            }
                        ),
                    '#' =>
                        Ok(
                            Cell {
                                foreground: ' ',
                                background: '#'
                            }
                        ),
                    ',' =>
                        Ok(
                            Cell {
                                foreground: ' ',
                                background: ',',
                            }
                        ),
                    '/' =>
                        Ok(
                            Cell {
                              foreground: ' ',
                              background: '/',
                            }
                        ),
                    ex =>
                        return Err(format!("Invalid Character used {}", ex)),
                }
            ).collect()
    }
}

#[derive(Builder, Clone, Debug, Eq, PartialEq)]
pub struct Map {
    name: &'static str,
    board: Vec<Vec<Cell>>
}

impl Map {
    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn board(&self) -> Vec<Vec<Cell>> {
        self.board.to_vec()
    }

    pub fn length(&self) -> usize {
        self.board.len()
    }

    pub fn width(&self) -> usize {
        self.board.first().unwrap().len()
    }
}

impl<'a> IntoIterator for &'a Map {
    type Item = Cell;
    type IntoIter = MapIntoIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        MapIntoIterator { map: self, x: 0, y: 0, direction: Direction::Horizontal }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Direction {
    Vertical,
    Horizontal,
}

#[derive(Debug, Eq, PartialEq)]
pub struct MapIntoIterator<'a> {
    map: &'a Map,
    x: usize,
    y: usize,
    direction: Direction,
}

impl<'a> MapIntoIterator<'a> {
    pub fn change_direction(&mut self, new_direction: Direction) {
        self.direction = new_direction;
    }

    pub fn direction(self) -> Direction {
        self.direction
    }
}

impl<'a> Iterator for MapIntoIterator<'a> {
    type Item = Cell;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x >= self.map.width() - 1 && self.y >= self.map.length() - 1{
            return None;
        }

        let result = self.map.board[self.y][self.x];

        match self.direction {
            Direction::Horizontal => {
                if self.x + 1 >= self.map.width() {
                    self.x = 0;
                    self.y += 1;
                }
                else {
                    self.x += 1;
                }
            },
            Direction::Vertical => {
                if self.y + 1 >= self.map.length() {
                    self.y = 0;
                    self.x += 1;
                }
                else {
                    self.y += 1;
                }
            }
        }

        Some(result)
    }
}

trait MapCompiler {
     fn compile_string<'a>(&self) -> Vec<Vec<Cell>>;
}

impl MapCompiler for str {
     fn compile_string<'a>(&self) -> Vec<Vec<Cell>> {
        self
            .lines()
            .map( |line| CellBuilder::build_string(line).unwrap() )
            .collect()
     }
}

impl MapBuilder
{
    pub fn from_named(map_name : Option<MapName>) -> Result<Map, String> {
        match map_name {
            Some(map_val) => {
                MapBuilder::load_map(map_val)
            },
            None => MapBuilder::load_map(MapName::Tutorial),
        }
    }

    fn load_map(selected_map: MapName) -> Result<Map, String>{
        match selected_map {
            MapName::Angled => MapBuilder::default()
                .name("Angled")
                .board(ANGLED.compile_string())
                .build()
            ,
            MapName::Danger => MapBuilder::default()
                .name("Danger")
                .board(DANGER.compile_string())
                .build()
            ,
            MapName::Reverse => MapBuilder::default()
                .name("Reverse")
                .board(REVERSE.compile_string())
                .build()
            ,
            MapName::Spiral => MapBuilder::default()
                .name("Spiral")
                .board(SPIRAL.compile_string())
                .build()
            ,
            MapName::Wiggler => MapBuilder::default()
                .name("Wiggler")
                .board(WIGGLER.compile_string())
                .build()
            ,
            _ => MapBuilder::default()
                .name("Tutorial")
                .board(TUTORIAL.compile_string())
                .build()
            ,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{MapBuilder, CellBuilder, Direction, MapName};

    #[test]
    // Nerd Note: The Tutorial Map is 34X12
    fn length_and_width() {
        let test = MapBuilder::from_named(None);

        assert!({
            match test.clone() {
                Err(ex) => panic!(ex),
                Ok(map_val) => {
                    match map_val.length() {
                        12 => true,
                        _ => false
                    }
                },
            }
        });

        assert!({
            match test.clone() {
                Err(ex) => panic!(ex),
                Ok(map_val) => {
                    match map_val.width() {
                        34 => true,
                        _ => false
                    }
                },
            }
        });
    }

    #[test]
    fn invalid_characters_fallback() {
        let test = CellBuilder::build_string("How Does This Not Exist");

        assert!(test.is_err() && test == Err("Invalid Character used o".to_string()));
    }

    #[test]
    fn builder_test() {

        let m_test = match MapBuilder::from_named(None) {
            Ok(map) => map,
            Err(ex) => panic!(ex),
        };

        print!("{:?}", m_test);
    }

    #[test]
    fn first_character_from_iterator_test() {
        let test = MapBuilder::from_named(None).unwrap();

        let test_iter = test.into_iter().next().unwrap();

        assert!(test_iter.background == 'H');
    }

    #[test]
    #[should_panic]
    fn all_characters_are_not_h() {
        let test = MapBuilder::from_named(None).unwrap();

        for chara in test.into_iter() {
            assert!(chara.background == 'H');
        }
    }

    #[test]
    fn change_iterator_direction() {
        let test = MapBuilder::from_named(None).unwrap();

        let mut test_iter = test.into_iter();

        test_iter.change_direction(Direction::Vertical);

        assert!(test_iter.direction() == Direction::Vertical);
    }

    #[test]
    fn check_vertical_versus_horizontal_map() {
        let test = MapBuilder::from_named(None).unwrap();

        let mut test_iter = test.into_iter();

        test_iter.change_direction(Direction::Vertical);

        for x in 1..75 {
            test_iter.next();
        }

        assert!(test_iter.next().unwrap().background == '|' && test_iter.next().unwrap().background == '|');

        test_iter = test.into_iter();

        for x in 1..75 {
            test_iter.next();
        }

        assert!(test_iter.next().unwrap().background == '|' && test_iter.next().unwrap().background == '_');
    }

    #[test]
    fn iterate_horizontally() {
        let mut idx = 1;

        for _ in MapBuilder::from_named(None).unwrap().into_iter() { idx = idx + 1; }

        assert_eq!(idx, 408);
    }

    #[test]
    fn iterate_vertically() {
        let mut idx = 1;

        let elems = MapBuilder::from_named(None).unwrap();

        let mut map_iter = elems.into_iter();

        map_iter.change_direction(Direction::Vertical);

        for _ in map_iter { idx = idx + 1; }

        assert_eq!(idx, 408);
    }

    // Noticed I'd missed '/' ^^;
    #[test]
    fn build_all_maps()
    {
      let MapDefault = MapBuilder::from_named(None).unwrap();
      let MapAngled = MapBuilder::from_named(Some(MapName::Angled)).unwrap();
      let MapDanger = MapBuilder::from_named(Some(MapName::Danger)).unwrap();
      let MapReverse = MapBuilder::from_named(Some(MapName::Reverse)).unwrap();
      let MapSpiral = MapBuilder::from_named(Some(MapName::Spiral)).unwrap();
      let MapWiggler = MapBuilder::from_named(Some(MapName::Wiggler)).unwrap();
    }
}
