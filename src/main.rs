mod map;
mod map_parser;

use map::Cell;

fn main() {
	let map = vec![
					vec![Cell::WALL, Cell::SPACE, Cell::WALL],
					vec![Cell::SPACE, Cell::SPACE, Cell::WALL],
					vec![Cell::SPACE, Cell::WALL, Cell::WALL],
			];

	let parser = map_parser::Parser::new();
	println!("{:?}", parser.parse(&map));
}

