use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::{IntoNodeIdentifiers, Bfs, Visitable, VisitMap};
use std::collections::HashMap;
use std::collections::VecDeque;
use rayon::prelude::*;
use std::collections::HashSet;
pub fn closeness_centrality(graph: &DiGraph<(), ()>, node: NodeIndex, max_path_length: usize) -> Option<f64> {
    let node_count = graph.node_count() as f64;
    let mut visited = graph.visit_map();
    let mut bfs = Bfs::new(graph, node);
    let mut total_distance = 0.0;
    let mut distances = HashMap::new();
    distances.insert(node, 0);

    while let Some(nx) = bfs.next(graph) {
        if let Some(&d) = distances.get(&nx) {
            for neighbor in graph.neighbors(nx) {
                if !visited.is_visited(&neighbor) {
                    let dist = d + 1;
                    if dist <= max_path_length {
                        visited.visit(neighbor);
                        distances.insert(neighbor, dist);
                        total_distance += dist as f64;
                    }
                }
            }
        }
    }
    if total_distance == 0.0 {
        return None; // This handles the case where a node is disconnected from others
    }
    Some((node_count - 1.0) / total_distance)
}
#[cfg(test)]
pub fn closeness_centrality_subset(graph: &DiGraph<(), ()>, nodes_subset: &HashSet<NodeIndex>, max_path_length: usize) -> HashMap<NodeIndex, Option<f64>> {
    nodes_subset.iter()
        .map(|&node| (node, closeness_centrality(graph, node, max_path_length)))
        .collect()
}
//I only put that one above as a test and the rest below are not tests, unless marked otherwise. I did that to get rid of a warning I kept receiving.
pub fn betweenness_centrality(graph: &DiGraph<(), ()>) -> HashMap<NodeIndex, f64> {
    let node_indices: Vec<NodeIndex> = graph.node_indices().collect();
    node_indices
         .par_iter()
         .map(|&s| calculate_betweenness_for_node(graph, s))
         .reduce_with(merge_centralities)
         .unwrap_or_default()
}
pub fn calculate_betweenness_for_node(graph: &DiGraph<(), ()>, s: NodeIndex) -> HashMap<NodeIndex, f64> {
    let mut centralities = HashMap::new();
    let _node_count = graph.node_count();
    let mut sp_count: HashMap<NodeIndex, i64> = HashMap::new();
    let mut sp_length: HashMap<NodeIndex, i64> = HashMap::new();
    let mut pred: HashMap<NodeIndex, Vec<NodeIndex>> = HashMap::new();
    for v in graph.node_identifiers() {
        pred.insert(v, Vec::new());
        sp_count.insert(v, 0);
        sp_length.insert(v, -1);
    }
    sp_count.insert(s, 1);
    sp_length.insert(s, 0);
    let mut queue = VecDeque::new();
    queue.push_back(s);
    while let Some(v) = queue.pop_front() {
        for neighbor in graph.neighbors(v) {
            if *sp_length.get(&neighbor).unwrap() == -1 {
                queue.push_back(neighbor);
                sp_length.insert(neighbor, sp_length[&v] + 1);
            }
            if sp_length[&neighbor] == sp_length[&v] + 1 {
                *sp_count.get_mut(&neighbor).unwrap() += sp_count[&v];
                pred.get_mut(&neighbor).unwrap().push(v);
            }
        }
    }
    let mut delta: HashMap<NodeIndex, f64> = HashMap::new();
    for v in graph.node_identifiers() {
        delta.insert(v, 0.0);
    }
    let mut stack: Vec<NodeIndex> = Vec::new();
    while let Some(w) = queue.pop_front() {
        stack.push(w);
        for v in &pred[&w] {
            let c = delta[v] + (sp_count[v] as f64 / sp_count[&w] as f64) * (1.0 + delta[&w]);
            delta.insert(*v, c);
        }
    }
    for &w in stack.iter().rev() {
        if w != s {
            *centralities.entry(w).or_insert(0.0) += delta[&w];
        }
    }
    centralities
}
pub fn merge_centralities(mut acc: HashMap<NodeIndex, f64>, local_centralities: HashMap<NodeIndex, f64>) -> HashMap<NodeIndex, f64> {
    for (node, centrality) in local_centralities {
        *acc.entry(node).or_insert(0.0) += centrality;
    }
    acc
}

        #[cfg(test)]
        mod tests {
            use super::*;
            use petgraph::graph::{DiGraph, NodeIndex};
            use std::collections::HashSet;
        
            // Creates a simple directed graph for testing
            fn create_test_graph() -> DiGraph<(), ()> {
                let mut graph = DiGraph::new();
                // Add nodes
                let n0 = graph.add_node(()); 
                let n1 = graph.add_node(()); 
                let n2 = graph.add_node(()); 
                // Add edges
                graph.add_edge(n0, n1, ()); 
                graph.add_edge(n1, n2, ()); 
                graph.add_edge(n2, n0, ()); 
                graph
            }
        
            #[test]
            fn test_closeness_centrality() {
                let graph = create_test_graph();
                let max_path_length = 7;
                let centrality = closeness_centrality(&graph, NodeIndex::new(0), max_path_length);
                let expected_value = 0.3333333333333333;
                assert!(centrality.is_some(), "Closeness centrality for Node 0 should not be None");
                assert_eq!(centrality.unwrap(), expected_value, "Closeness centrality for Node 0 should be {}", expected_value);
            }
        
            #[test]
            fn test_betweenness_centrality() {
                let graph = create_test_graph();
                let centralities = betweenness_centrality(&graph);
                let expected_values = vec![0.0, 0.0, 0.0];
                for (i, &expected) in expected_values.iter().enumerate() {
                    let centrality = centralities.get(&NodeIndex::new(i)).unwrap_or(&0.0);
                    assert_eq!(*centrality, expected, "Betweenness centrality for Node {} should be {}", i, expected);
                }
            }
        
            #[test]
            fn test_closeness_centrality_subset() {
                let graph = create_test_graph();
                let mut subset = HashSet::new();
                subset.insert(NodeIndex::new(0));
                subset.insert(NodeIndex::new(1));
                let max_path_length = 7;
                let centralities = closeness_centrality_subset(&graph, &subset, max_path_length);
                println!("Subset: {:?}", subset);
                println!("Centralities: {:?}", centralities);
                assert_eq!(centralities.len(), subset.len(), "Should calculate centrality for subset nodes");
                for node in &subset {
                    assert!(
                        centralities.contains_key(node), 
                        "Centrality should be calculated for node {:?} in subset",
                        node
                    );
                    assert!(
                        centralities[node].is_some(), 
                        "Centrality for node {:?} in subset should not be None",
                        node
                    );
                }
            }            
        }