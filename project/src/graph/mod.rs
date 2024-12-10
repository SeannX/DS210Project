use std::collections::HashMap;

// Edge struct used to represent a specific edge in the graph
#[derive(Debug, Clone)]
pub struct Edge {
    pub from: usize,       // Source node that this edge is from
    pub to: usize,         // Node that this edge points to
    pub weight: f64,       // Weight of the edge.
    pub timestamp: u64,    // Timestamp associated with this edge.
}

// Grpah struct - corresponds to a complete graph
// represents by a hashtable with the node as the key 
// and the Edge struct containing the information of
pub struct Graph { pub content: HashMap<usize, Vec<Edge>> }

// function that transform a list of Edge struct to Graph
pub fn mk_graph(edge_lst: &Vec<Edge>) -> Graph {
    let mut graph_hashmap: HashMap<usize, Vec<Edge>> = HashMap::new();
    for edge in edge_lst.iter() {
        let source_node: usize = edge.from;
        // Add the Edge to the vector of source node if source node already exists, 
        graph_hashmap.entry(source_node).or_insert(Vec::new()).push(edge.clone());
    }
    return Graph{ content: graph_hashmap };
}
