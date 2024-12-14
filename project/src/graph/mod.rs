use std::collections::HashMap;
use std::collections::HashSet;

pub mod algorithm;
use algorithm::bfs;
// use algorithm::dijkstra;

// Edge struct used to represent a specific edge in the graph
// the forth field - TimeStamp is removed since not using.
#[derive(Debug, Clone)]
pub struct Edge {
    pub from: usize,       // Source node that this edge is from
    pub to: usize,         // Node that this edge points to
    pub weight: f64,       // Weight of the edge.
}

#[derive(Debug, Clone)]
pub struct NodeNeighbors {
    pub input_nodes: Vec<usize>,
    pub output_nodes: Vec<usize>,
}

// Grpah struct - corresponds to a complete graph
// represents by a hashtable with the node as the key 
// and the Edge struct containing the information of
#[derive(Debug, Clone)]
pub struct Graph { pub content: HashMap<usize, Vec<Edge>> }

impl Graph {
    // Constructor that transform a list of Edge struct to Graph
    pub fn new(edge_lst: &Vec<Edge>) -> Graph {
        let mut graph_hashmap: HashMap<usize, Vec<Edge>> = HashMap::new();

        for edge in edge_lst.iter() {
            // Add edge to the source node's adjacency list
            graph_hashmap.entry(edge.from).or_insert(Vec::new()).push(edge.clone());
            
            // Ensure the target node is also included in the graph, even if it has no outgoing edges
            graph_hashmap.entry(edge.to).or_insert(Vec::new());
        }

        Graph { content: graph_hashmap }
    }

    // helper method that get the list of neighbors of a node.
    fn get_neighbors(&self, node: usize) -> NodeNeighbors {
        let mut input_nodes: Vec<usize> = Vec::new();
        let mut output_nodes: Vec<usize> = Vec::new();

        if let Some(edges) = self.content.get(&node) {
            for edge in edges {
                output_nodes.push(edge.to);
            }
        }

        for (source_node, edges) in &self.content {
            for edge in edges {
                if edge.to == node {
                    input_nodes.push(*source_node);
                }
            }
        }

        return NodeNeighbors{ input_nodes: input_nodes, output_nodes: output_nodes };
    }


    // compute the indegree and outdegree of ALL nodes in the graph.
    // return value: (indegree HashMap, outdegre HashMap)
    pub fn get_degrees(&self) -> (HashMap<usize, f64>, HashMap<usize, f64>) {
        let mut in_degree = HashMap::new();
        let mut out_degree = HashMap::new();

        for node in self.content.keys() {
            let neighbors = self.get_neighbors(*node);

            out_degree.insert(*node, neighbors.output_nodes.len() as f64);

            in_degree.insert(*node, neighbors.input_nodes.len() as f64);
        }
        return (in_degree, out_degree);
    }

    // This function calculates the clustering coefficient of a given node.
    // Clustering coefficient "is a measure of the degree to which nodes 
    // in a graph tend to cluster together." - Wikipedia.
    // Its formula is cc(n) = 2 * total edges between neighbors of n / # neighbor * (# neighbor - 1)
    pub fn clustering_coefficient(&self, node: usize) -> f64 {
        let neighbors = self.get_neighbors(node);

        let mut edges_btw_nb = 0;
        let mut nb_set: HashSet<usize> = HashSet::new();

        for nb in neighbors.output_nodes.iter() {
            nb_set.insert(nb.clone());
        }
        for nb in neighbors.input_nodes.iter() {
            nb_set.insert(nb.clone());
        }

        // total edges between neighbors of n
        for &neighbor in &nb_set {
            if let Some(edges) = self.content.get(&neighbor) {
                for edge in edges {
                    if nb_set.contains(&edge.to) {
                        edges_btw_nb += 1; // Count only directed edges between neighbors
                    }
                }
            }
        }

        let num_nb = nb_set.len();
        if num_nb < 2 {
            return 0.0;
        }

        let possible_connections = num_nb * (num_nb - 1);

        return edges_btw_nb as f64 / possible_connections as f64
    }

    // Finds all connected subgraphs in the graph
    pub fn find_subgraphs(&self) -> Vec<Graph> {
        let mut visited = HashSet::new();
        let mut subgraphs = Vec::new();

        for &node in self.content.keys() {
            if !visited.contains(&node) {
                // Use BFS to find all nodes in the connected subgraph
                let subgraph_nodes = bfs(&self, node, &mut visited);

                // Construct the subgraph from the collected nodes
                let mut subgraph_content = HashMap::new();
                for subgraph_node in subgraph_nodes.iter() {
                    if let Some(edges) = self.content.get(subgraph_node) {
                        subgraph_content.insert(*subgraph_node, edges.clone());
                    }
                }

                subgraphs.push(Graph { content: subgraph_content });
            }
        }

        return subgraphs;
    }

    // Calculate the trust score of a given node
    // this is determining by the sum of the
    // weight of the indegree edges divided by
    // the number of total indegrees.
    pub fn get_trust_score(&self, node: usize) -> f64 {
        let mut total_trust_score = 0.0;
        let mut node_count = 0;

        for (_, edges) in &self.content {
            for edge in edges {
                if edge.to == node {
                    total_trust_score += edge.weight;
                    node_count += 1;
                }
            }
        }

        if node_count == 0 {
            return 0.0;
        }

        return total_trust_score as f64 / node_count as f64
    }

}

// ----------------------- TESTS ----------------------- 

#[test]
fn test_graph_new_with_edges() {
    let edges = vec![
        Edge { from: 1, to: 2, weight: 1.0 },
        Edge { from: 2, to: 3, weight: 2.0 },
    ];
    let graph = Graph::new(&edges);

    assert_eq!(graph.content.len(), 3); // Nodes 1, 2, 3
    assert!(graph.content.contains_key(&1));
    assert!(graph.content.contains_key(&2));
    assert!(graph.content.contains_key(&3));
}

#[test]
fn test_graph_include_target_nodes() {
    let edges = vec![
        Edge { from: 1, to: 2, weight: 1.0 },
    ];
    let graph = Graph::new(&edges);

    assert!(graph.content.contains_key(&2)); // Ensure target node 2 is included
    assert_eq!(graph.content[&2].len(), 0); // Node 2 has no outgoing edges
}

#[test]
fn test_get_neighbors() {
    let edges = vec![
        Edge { from: 1, to: 2, weight: 1.0 },
        Edge { from: 3, to: 2, weight: 1.0 },
    ];
    let graph = Graph::new(&edges);

    let neighbors = graph.get_neighbors(2);
    assert_eq!(neighbors.input_nodes.contains(&1) || neighbors.input_nodes.contains(&3), true);
    assert_eq!(neighbors.output_nodes, vec![]);
}

#[test]
fn test_get_neighbors_no_edges() {
    let edges = vec![
        Edge { from: 1, to: 2, weight: 1.0 },
    ];
    let graph = Graph::new(&edges);

    let neighbors = graph.get_neighbors(3);
    assert!(neighbors.input_nodes.is_empty());
    assert!(neighbors.output_nodes.is_empty());
}

#[test]
fn test_get_degrees() {
    let edges = vec![
        Edge { from: 1, to: 2, weight: 1.0 },
        Edge { from: 2, to: 3, weight: 1.0 },
    ];
    let graph = Graph::new(&edges);

    let (in_degree, out_degree) = graph.get_degrees();
    assert_eq!(in_degree[&2], 1.0); // Node 2 has 1 incoming edge
    assert_eq!(out_degree[&2], 1.0); // Node 2 has 1 outgoing edge
}

#[test]
fn test_get_degrees_no_outgoing_or_incoming() {
    let edges = vec![
        Edge { from: 1, to: 2, weight: 1.0 },
    ];
    let graph = Graph::new(&edges);

    let (in_degree, out_degree) = graph.get_degrees();
    println!("{:?}, {:?}", in_degree, out_degree);
    assert_eq!(in_degree[&1], 0.0); // Node 1 has no incoming edges
    assert_eq!(out_degree[&2], 0.0); // Node 2 has no outgoing edges
}

#[test]
fn test_clustering_coefficient_connected() {
    let edges = vec![
        Edge { from: 1, to: 2, weight: 1.0 },
        Edge { from: 2, to: 3, weight: 1.0 },
        Edge { from: 3, to: 1, weight: 1.0 },
    ];
    let graph = Graph::new(&edges);

    let cc = graph.clustering_coefficient(2);
    println!("{:?}", graph);
    println!("{}", cc);

    assert!(cc > 0.0); // Node 2's neighbors (1 and 3) are connected so cc should be > 0 
}

#[test]
fn test_clustering_coefficient_no_neighbors() {
    let edges = vec![
        Edge { from: 1, to: 2, weight: 1.0 },
    ];
    let graph = Graph::new(&edges);

    let cc = graph.clustering_coefficient(3);
    assert_eq!(cc, 0.0); // Node 3 has no neighbors
}

// Entire graph is connected
#[test]
fn test_find_one_subgraphs() {
    let edges = vec![
        Edge { from: 1, to: 2, weight: 1.0 },
        Edge { from: 2, to: 3, weight: 1.0 },
    ];
    let graph = Graph::new(&edges);

    let subgraphs = graph.find_subgraphs();
    assert_eq!(subgraphs.len(), 1);
}

#[test]
fn test_find_subgraphs() {
    let edges = vec![
        Edge { from: 1, to: 2, weight: 1.0 },
        Edge { from: 3, to: 4, weight: 1.0 },
    ];
    let graph = Graph::new(&edges);

    let subgraphs = graph.find_subgraphs();
    assert_eq!(subgraphs.len(), 2); // Two disconnected subgraphs
}

#[test]
fn test_get_trust_score() {
    let edges = vec![
        Edge { from: 1, to: 2, weight: 1.5 },
        Edge { from: 3, to: 2, weight: 2.5 },
    ];
    let graph = Graph::new(&edges);

    let trust_score = graph.get_trust_score(2);
    assert_eq!(trust_score, 2.0); // Average weight: (1.5 + 2.5) / 2
}

#[test]
fn test_get_trust_score_zero_indegree() {
    let edges = vec![
        Edge { from: 1, to: 2, weight: 1.5 },
    ];
    let graph = Graph::new(&edges);

    let trust_score = graph.get_trust_score(3);
    assert_eq!(trust_score, 0.0); // No incoming edges for node 3
}
