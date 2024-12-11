pub mod graph::Graph

use std::collections::HashSet;
use std::collections::VecDeque;

// DFS algorithm
pub fn dfs(&Graph, start_node: usize) -> Vec<usize> {
    let mut visited = HashSet::new();
    let mut nodes_to_visit = vec![start_node];
    let mut connected_nodes = Vec::new();

    while let Some(node) = nodes_to_visit.pop() {
        if !visited.contains(&node) {
            visited.insert(node);
            result.push(node);

        if let Some(edges) = self.content.get(&node) {
            for edge in edges {
                nodes_to_visit.push(edge.to);
            }
            }
        }
    }
    return connected_nodes
}

// BFS algorithm
pub fn bfs(&self, start_node: usize) -> Vec<(usize, usize)> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut distances: Vec<(usize, usize)> = Vec::new();

    queue.push_back((start_node, 0)); // start node at layer 0
    visited.insert(start_node);

    while let Some((node, dis)) = queue.pop_front() {
        distances.push((node, dis));

        if let Some(edges) = self.content.get(&node) {
            for edge in edges {
                if !visited.contains(&edge.to) {
                    visited.insert(edge.to);
                    queue.push_back((edge.to, dis + 1));
                }
            }
        }
    }
    return distances;
}




