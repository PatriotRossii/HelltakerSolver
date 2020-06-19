use petgraph::Graph;
use std::collections::HashMap;

use crate::map;

#[derive(PartialEq, Eq, Hash)]
struct Node<'a> {
	position: (usize, usize),
	r#type: &'a map::Cell,
}

fn graph_eq<N, E, Ty, Ix>(
    a: &petgraph::Graph<N, E, Ty, Ix>,
    b: &petgraph::Graph<N, E, Ty, Ix>,
) -> bool
where
    N: PartialEq,
    E: std::hash::Hash + std::cmp::Eq + PartialEq,
    Ty: petgraph::EdgeType,
    Ix: petgraph::graph::IndexType + PartialEq,
{
	use std::collections::HashSet;
    let a_ns = a.raw_nodes().iter().map(|n| &n.weight);
    let b_ns = b.raw_nodes().iter().map(|n| &n.weight);


    let a_es: HashSet<_> = a.raw_edges().iter().map(|e| HashSet::new().insert((e.source(), e.target(), &e.weight))).collect();
    let b_es: HashSet<_> = b.raw_edges().iter().map(|e| HashSet::new().insert((e.source(), e.target(), &e.weight))).collect();
    a_ns.eq(b_ns) && a_es.eq(&b_es)
}

pub struct Parser { }
impl Parser {
	pub fn new() -> Parser { Parser{} }
	pub fn parse<'a>(&self, map: &'a map::Map) -> Graph<&'a map::Cell,i32,petgraph::Undirected> {
		let mut graph = Graph::<&map::Cell, i32, petgraph::Undirected>::new_undirected();
		let mut nodes = HashMap::new();
		let walkable = map::Map::walkable();

		let map = map.data();
		let rows = map.len();
		let columns = map[0].len();

		for row in 0..rows {
			for cell in 0..columns {
				if walkable.contains(&map[row][cell]) {
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
				if walkable.contains(&map[row - 1][column]) {
					let node_edge_with = nodes.iter().find(|&node| node.0.position == (row - 1, column));
					graph.update_edge(nodes[cell], nodes[node_edge_with.unwrap().0], 1);
				}
			}
			if column != 0 { 
				if walkable.contains(&map[row][column - 1]) {
					let node_edge_with = nodes.iter().find(|&node| node.0.position == (row, column - 1));
					graph.update_edge(nodes[cell], nodes[node_edge_with.unwrap().0], 1); 
				}
			}
			if row < rows {
				if walkable.contains(&map[row + 1][column]) {
					let node_edge_with = nodes.iter().find(|&node| node.0.position == (row + 1, column));
					graph.update_edge(nodes[cell], nodes[node_edge_with.unwrap().0], 1); 
				}
			}
			if column < columns {
				if walkable.contains(&map[row][column + 1]) {
					let node_edge_with = nodes.iter().find(|&node| node.0.position == (row, column + 1));
					graph.update_edge(nodes[cell], nodes[node_edge_with.unwrap().0], 1);
				}
			}
		}
		graph
	}
}

#[cfg(test)]
mod tests {
    #[test]
fn test_parsing_map() {
    	use {crate::{map::{self, Cell}, map_parser::{self, graph_eq}}, petgraph::Graph};

    	let vec_map = vec![
    		vec![Cell::WALL, Cell::START, Cell::WALL],
    		vec![Cell::SPACE, Cell::SPACE, Cell::WALL],
    		vec![Cell::FINISH, Cell::WALL, Cell::WALL],
    	];
    	let map = map::Map::from_vec(&vec_map);


		let mut expected = Graph::<&map::Cell, i32, petgraph::Undirected>::new_undirected();
		{
			let n0 = expected.add_node(&vec_map[0][1]);
			let n1 = expected.add_node(&vec_map[1][0]);
			let n2 = expected.add_node(&vec_map[1][1]);
			let n3 = expected.add_node(&vec_map[2][0]);

			expected.add_edge(n0, n2, 1);
			expected.add_edge(n3, n1, 1);
			expected.add_edge(n1, n2, 1);
		}

		let got = map_parser::Parser::new().parse(&map);
		assert!(graph_eq(&got, &expected));
    }
}