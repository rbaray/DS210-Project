use petgraph::Direction::Incoming;
use petgraph::graph::NodeIndex;
use petgraph::visit::{Bfs, EdgeRef, Visitable};
use petgraph::Graph;
use std::collections::HashMap;
pub fn bfs(graph: &Graph<(), ()>, start: NodeIndex<u32>) -> HashMap<NodeIndex<u32>, usize> {
    let mut distances = HashMap::new();
    let mut bfs = Bfs::new(graph, start);
    let mut _visit_map = graph.visit_map();
    distances.insert(start, 0);
    while let Some(nx) = bfs.next(&graph) {
        if nx != start{
            let mut distance = None;
            for edge in graph.edges_directed(nx, Incoming) { 
                if let Some(&parent_distance) = distances.get(&edge.source()){
                    let new_distance = parent_distance + 1;
                    if distance.map_or(true, |d| new_distance < d) {
                        distance = Some(new_distance);
                    }
                }
            }
        match distance {
            Some(dist) => {
                distances.insert(nx, dist);
            }
            None => {
                eprintln!("Node {:?} is not reachable from the start node.", nx);
                distances.insert(nx,usize::MAX);
                }
            }
        }
    }
    distances
}

#[cfg(test)]
mod tests {
    use super::*;
    use petgraph::graph::DiGraph;

    #[test]
    fn test_bfs() {
        let mut graph = DiGraph::new();
        let n1 = graph.add_node(());
        let n2 = graph.add_node(());
        let n3 = graph.add_node(());
        let n4 = graph.add_node(());

        graph.add_edge(n1, n2, ());
        graph.add_edge(n1, n3, ());
        graph.add_edge(n2, n4, ());
        graph.add_edge(n3, n4, ());

        let distances = bfs(&graph, n1);

        assert_eq!(distances[&n1], 0, "Distance from start to itself should be 0");
        assert_eq!(distances[&n2], 1, "Distance from start to node 2 should be 1");
        assert_eq!(distances[&n3], 1, "Distance from start to node 3 should be 1");
        assert_eq!(distances[&n4], 2, "Distance from start to node 4 should be 2");
    }
}
