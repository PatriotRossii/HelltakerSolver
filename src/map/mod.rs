
use std::collections::HashMap;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Cell {
	WALL,
	SPACE,
	FINISH,
	START,
	TRAP,
}

#[derive(PartialEq, Debug)]
pub struct Map{ data: Vec<Vec<Cell>> }
impl Map {
	pub fn walkable() -> Vec<Cell> {
		return vec![Cell::SPACE, Cell::START, Cell::FINISH, Cell::TRAP];
	}
	pub fn encoding() -> HashMap<char, Cell> {
		vec![('w', Cell::WALL), ('e', Cell::SPACE), ('f', Cell::FINISH), ('s', Cell::START), ('t', Cell::TRAP)].into_iter().collect()
	}
	pub fn cost() -> HashMap<Cell, i32> {
		vec![(Cell::SPACE, 1), (Cell::TRAP, 2), (Cell::START, 1), (Cell::FINISH, 1)].into_iter().collect()
	}
	pub fn from_file(path: &str) -> Map {
		let file = match File::open(path) {
			Err(err) => panic!("couldn't open {}: {}", path, err),
			Ok(file) => file,
		};
		let file = BufReader::new(file);

		let encoding = Map::encoding();
		let map = file.lines().into_iter().map(|x| x.unwrap().chars().into_iter().map(|y| encoding[&y]).collect()).collect();
		Map{data: map}
	}	
	pub fn from_vec(vec: &Vec<Vec<Cell>>) -> Map {
		Map{data: vec.to_vec()}
	}
	pub fn data(&self) -> &Vec<Vec<Cell>> {
		return &self.data
	}
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_loading_example_map() {
    	use crate::map::{self, Cell};

    	let vec_map = vec![
    		vec![Cell::WALL, Cell::START, Cell::WALL],
    		vec![Cell::SPACE, Cell::SPACE, Cell::WALL],
    		vec![Cell::FINISH, Cell::WALL, Cell::WALL],
    	];

        let got = map::Map::from_file("test/maps/example_map");
        let expected = map::Map::from_vec(&vec_map);
        assert_eq!(got, expected);
    }
    #[test]
    fn test_loading_example_trap_map() {
    	use crate::map::{self, Cell};

    	let vec_map = vec![
    		vec![Cell::WALL, Cell::START, Cell::WALL],
    		vec![Cell::SPACE, Cell::TRAP, Cell::WALL],
    		vec![Cell::FINISH, Cell::WALL, Cell::WALL],
    	];

    	let got = map::Map::from_file("test/maps/example_trap_map");
    	let expected = map::Map::from_vec(&vec_map);
    	assert_eq!(got, expected);
    }
}