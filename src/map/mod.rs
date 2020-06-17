#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Cell {
	WALL,
	SPACE,
	FINISH,
	START,
}

pub struct Map{}
impl Map {
	pub fn walkable() -> Vec<Cell> {
		return vec![Cell::SPACE, Cell::START, Cell::FINISH];
	}
}