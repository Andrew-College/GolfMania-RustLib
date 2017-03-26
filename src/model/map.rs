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
            let mut result = input
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
                                    foreground: '|',
                                    background: '|'
                                }
                            ),
                        '_' =>
                            Ok (
                                Cell {
                                    foreground: '_',
                                    background: '_'
                                }
                            ),
                        '\\' =>
                            Ok (
                                Cell {
                                    foreground: '\\',
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
                        ex =>
                            Err(format!("Invalid Character used {}", ex))
                    }
                );
            
            //let count = result.take_while(Result::is_ok).count();
            
            // Jump out early on error
            for elem in &mut result {
                match elem {
                    Err(e) => {
                        println!("There was an error: {}", e);
                        return Err(e);
                    },
                    _ => {},
                }
            }

            Ok(result.map( |goodch| goodch.unwrap() ).collect())
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
            // MapName::Angled => MapBuilder::default()
            //     .name("Angled")
            //     .board(ANGLED.compile_string())
            //     .build()
            // ,
            // MapName::Danger => MapBuilder::default()
            //     .name("Angled")
            //     .board(ANGLED.compile_string())
            //     .build()
            // ,
            // MapName::Reverse => MapBuilder::default()
            //     .name("Angled")
            //     .board(ANGLED.compile_string())
            //     .build()
            // ,
            // MapName::Spiral => MapBuilder::default()
            //     .name("Angled")
            //     .board(ANGLED.compile_string())
            //     .build()
            // ,
            // MapName::Wiggler => MapBuilder::default()
            //     .name("Angled")
            //     .board(ANGLED.compile_string())
            //     .build()
            // ,
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
    use super::{MapBuilder, CellBuilder};

    // #[test]
    // fn firstbackground_char_of_tutorial() {
    //     // let test = MapBuilder::from_named(None);

    //     // assert!({
    //     //     match test {
    //     //         Err(ex) => panic!("keems"),
    //     //         Ok(map_val) => {
    //     //             let the_board = map_val.board;
    //     //             match the_board.first() {
    //     //                 Some(column) => column.first().unwrap().background == 'H',
    //     //                 otherthing => panic!(otherthing),
    //     //             }
    //     //         },
    //     //     }
    //     // });

    //     // assert_eq!(
    //     //     test
    //     //         .unwrap()
    //     //         .board
    //     //             .first()
    //     //                 .unwrap()
    //     //             .first()
    //     //                 .unwrap()
    //     //         .background, 
    //     //     'H'
    //     // );
    // }

    #[test]
    fn invalid_characters_fallback() {
        let test = CellBuilder::build_string("memes");

        for test_cell in test {
            assert_eq!(test_cell.first().unwrap().background, ',');
        }
    }

    #[test]
    fn builder_test() {

        let m_test = match MapBuilder::from_named(None) {
            Ok(map) => map,
            Err(ex) => panic!(ex),
        };

        print!("{:?}", m_test);
    }
}