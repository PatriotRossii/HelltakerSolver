use petgraph::Graph;
use petgraph::algo::astar;

use crate::map;

pub struct Analyzer {}
impl Analyzer {
	pub fn new() -> Analyzer { Analyzer{} }
	pub fn analyze<'a>(&self, graph: &'a Graph<&map::Cell,i32,petgraph::Undirected>) -> Option<(i32, std::vec::Vec<petgraph::graph::NodeIndex>)> {
		let start = graph.node_indices().find(|x| if let map::Cell::START = graph[*x] { true } else { false }).unwrap();
		let finish = graph.node_indices().find(|x| if let map::Cell::FINISH = graph[*x] { true } else { false}).unwrap();
		let path = astar(&graph, start, |node| node == finish, |e| *e.weight(), |_| 0);
		return path;
	}
}