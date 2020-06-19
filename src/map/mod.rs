
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
}

#[derive(PartialEq, Debug)]
pub struct Map{ data: Vec<Vec<Cell>> }
impl Map {
	pub fn walkable() -> Vec<Cell> {
		return vec![Cell::SPACE, Cell::START, Cell::FINISH];
	}
	pub fn encoding() -> HashMap<char, Cell> {
		vec![('w', Cell::WALL), ('e', Cell::SPACE), ('f', Cell::FINISH), ('s', Cell::START)].into_iter().collect()
	}
	pub fn from_file(path: &str) -> Map {
		let file = match File::open(path) {
			Err(err) => panic!("couldn't open {}: {}", path, err),
			Ok(file) => file,
		};
		let file = BufReader::new(file);

		let encoding = Map::encoding();
		let mut map = Vec::new();
		for line in file.lines() {
			let mut row = Vec::new();
			line.unwrap().chars().for_each(|x| row.push(encoding[&x]));
			map.push(row)
		}
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
    fn test_loading() {
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
}