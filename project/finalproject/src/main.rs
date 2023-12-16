use std::time::Instant;
use std::io::Result;
use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef; 
mod alg; 
mod read;
mod alg2;
use alg::bfs;
use read::{node_count, edge_count, print_summary};
use crate::alg2::closeness_centrality; 
fn main() -> Result<()> {
    let verbose = true;
    println! ("Starting program");
    let start_time = Instant::now();
    let graph = read::parse_dataset("/Users/roamahbaray/Desktop/project/finalproject/src/Wiki-Vote.txt")?;
    println!("Parsed graph with {} nodes and {} edges", graph.node_count(), graph.edge_count());
    let start_node = NodeIndex::new(0);
    println!("Running BFS");
    let distances = bfs(&graph, start_node);
    let max_path_length = 7; 
    if let Some(node_centrality) = closeness_centrality(&graph, start_node, max_path_length) {
        println!("Closeness centrality of node {:?}: {}\n", start_node, node_centrality);
    } else {
        println!("Closeness centrality of node {:?} could not be calculated within path length {}\n", start_node, max_path_length);
    }
    if verbose{
        for node in graph.node_indices().take(3) {
            println!("Node {:?} has connections to", node);
            for edge in graph.edges(node).take(3) {
                println!("Edge from {:?} to {:?}", edge.source(), edge.target());
        }
    }
        for (node, &distance) in distances.iter().take(3) {
            println!("Node {:?} -> Node {:?}: {}", start_node, node, distance);
        }
    }
    println!("Calculating centralities...");
    let centralities = alg2::betweenness_centrality(&graph);
    for (node, centrality) in centralities.iter().enumerate().take(7) {
        println!("Betweenness centrality of node {:?}: {:?}", node, centrality);
    }
    println!("Printing summary...");
    print_summary(&graph);
    println!("Number of nodes: {}", node_count(&graph));
    println!("Number of edges: {}", edge_count(&graph));
    let elapsed_time = start_time.elapsed();
    println!("Time elapsed for graph construction: {:?}", elapsed_time);
    Ok(())
}