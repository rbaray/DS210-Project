use petgraph::graph::DiGraph;
use petgraph::visit::EdgeRef;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::path::Path;
use std::collections::HashMap;
type PetGraph = DiGraph<(), ()>;
pub fn parse_dataset(file_path: &str) -> io::Result<PetGraph> {
    let path = Path::new(file_path);
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut graph = PetGraph::new();
    let mut index_map = HashMap::new();
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() == 2 {
            let src = parts[0].parse::<i32>().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            let dest = parts[1].parse::<i32>().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

            let src_index = *index_map.entry(src).or_insert_with(|| graph.add_node(()));
            let dest_index = *index_map.entry(dest).or_insert_with(|| graph.add_node(()));

            graph.add_edge(src_index, dest_index, ());
        }
    }
    Ok(graph)
}
pub fn node_count(graph: &PetGraph) -> usize {
    graph.node_count()
}

pub fn edge_count(graph: &PetGraph) -> usize {
    graph.edge_count()
}
pub fn print_summary(graph: &PetGraph) -> String {
    let mut summary = String::new();
    summary += &format!("Number of nodes: {}\n", graph.node_count());
    for node in graph.node_indices() {
        summary += &format!("Node {:?} has connections to:\n", node);
        for edge in graph.edges(node) {
            summary += &format!("Edge from {:?} to {:?}\n", edge.source(), edge.target());
        }
    }
    summary
}
