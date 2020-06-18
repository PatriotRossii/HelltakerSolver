mod map;
mod map_parser;
mod analyzer;


fn main() {
	let map = map::Map::from_file("test/maps/example_map");
	let parser = map_parser::Parser::new();
	let analyzer = analyzer::Analyzer::new();
	println!("{:?}", analyzer.analyze(&parser.parse(&map)));
}

