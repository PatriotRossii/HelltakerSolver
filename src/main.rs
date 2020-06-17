mod map;
mod map_parser;
mod analyzer;

use map::Cell;

fn main() {
	let map = vec![
					vec![Cell::WALL, Cell::START, Cell::WALL],
					vec![Cell::SPACE, Cell::SPACE, Cell::WALL],
					vec![Cell::FINISH, Cell::WALL, Cell::WALL],
			];

	let parser = map_parser::Parser::new();
	let analyzer = analyzer::Analyzer::new();
	println!("{:?}", analyzer.analyze(&parser.parse(&map)));
}

