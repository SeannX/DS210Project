use std::collections::HashSet;
use std::collections::VecDeque;

use crate::graph::Edge;
use crate::graph::Graph;

// BFS algorithm that finds all connected node in a subgraph
pub fn bfs(graph: &Graph, start_node: usize, visited: &mut HashSet<usize>) -> HashSet<usize> {
    let mut sub_graph_nodes = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back(start_node);
    visited.insert(start_node);

    while let Some(node) = queue.pop_front() {
        sub_graph_nodes.insert(node);

        if let Some(edges) = graph.content.get(&node) {
            for edge in edges {
                // Check both directions for undirected graph behavior
                if !visited.contains(&edge.to) {
                    visited.insert(edge.to);
                    queue.push_back(edge.to);
                }
                if !visited.contains(&edge.from) {
                    visited.insert(edge.from);
                    queue.push_back(edge.from);
                }
            }
        }
        // See if other node points to this edge
        for (other, edges) in &graph.content.clone() {
            for edge in edges {
                if edge.to == node && !visited.contains(&other) {
                    visited.insert(*other);
                    queue.push_back(*other);
                }
            }
        }
    }

    return sub_graph_nodes;
}

#[test]
fn test_bfs_single_connected_component() {
    let edges = vec![
        Edge { from: 1, to: 2, weight: 1.0 },
        Edge { from: 2, to: 3, weight: 1.0 },
        Edge { from: 3, to: 4, weight: 1.0 },
    ];
    let graph = Graph::new(&edges);
    let mut visited = HashSet::new();

    let sub_graph = bfs(&graph, 1, &mut visited);
    let expected: HashSet<usize> = vec![1, 2, 3, 4].into_iter().collect();

    assert_eq!(sub_graph, expected);
}

#[test]
fn test_bfs_disconnected_graph() {
    let edges = vec![
        Edge { from: 1, to: 2, weight: 1.0 },
        Edge { from: 2, to: 3, weight: 1.0 },
        Edge { from: 4, to: 5, weight: 1.0 },
    ];
    let graph = Graph::new(&edges);
    let mut visited = HashSet::new();

    let sub_graph = bfs(&graph, 1, &mut visited);
    let expected: HashSet<usize> = vec![1, 2, 3].into_iter().collect();

    assert_eq!(sub_graph, expected);
    assert!(!sub_graph.contains(&4));
    assert!(!sub_graph.contains(&5));
}
