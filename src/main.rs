use petgraph::Graph;
use petgraph::dot::{Dot, Config};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
enum Cell {
	WALL,
	SPACE
}

#[derive(PartialEq, Eq, Hash)]
struct Node<'a> {
	position: (usize, usize),
	r#type: &'a Cell,
}

fn main() {
	let mut graph = Graph::<&Cell, i32, petgraph::Undirected>::new_undirected();
	let mut nodes = HashMap::new();	
	let map = vec![
					vec![Cell::WALL, Cell::SPACE, Cell::WALL],
					vec![Cell::SPACE, Cell::SPACE, Cell::WALL],
					vec![Cell::SPACE, Cell::WALL, Cell::WALL],
			];
	
	let rows = map.len();
	let columns = map[0].len();

	for row in 0..rows {
		for cell in 0..columns {
			if let Cell::SPACE = &map[row][cell] {
				let node = Node{position: (row, cell), r#type: &map[row][cell]};
				nodes.insert(node, graph.add_node(&map[row][cell])); 
			}
		}
	}

	let rows = rows - 1;
	let columns = columns - 1;

	for cell in nodes.keys() {
		let (row, column) = cell.position;

		if row != 0 {
			if let Cell::SPACE = map[row - 1][column] {
				let node_edge_with = nodes.iter().find(|&node| node.0.position == (row - 1, column));
				graph.update_edge(nodes[cell], nodes[node_edge_with.unwrap().0], 1);
			}
		}
		if column != 0 { 
			if let Cell::SPACE = map[row][column - 1] {
				let node_edge_with = nodes.iter().find(|&node| node.0.position == (row, column - 1));
				graph.update_edge(nodes[cell], nodes[node_edge_with.unwrap().0], 1); 
			}
		}
		if row < rows {
			if let Cell::SPACE = map[row + 1][column] {
				let node_edge_with = nodes.iter().find(|&node| node.0.position == (row + 1, column));
				graph.update_edge(nodes[cell], nodes[node_edge_with.unwrap().0], 1); 
			}
		}
		if column < columns {
			if let Cell::SPACE = map[row][column + 1] {
				let node_edge_with = nodes.iter().find(|&node| node.0.position == (row, column + 1));
				graph.update_edge(nodes[cell], nodes[node_edge_with.unwrap().0], 1);
			}
		}
	}
}

