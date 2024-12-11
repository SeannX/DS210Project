use std::collections::HashMap;
use std::collections::HashSet;

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
pub struct Graph { pub content: HashMap<usize, Vec<Edge>> }

impl Graph {
    // Constructor that transform a list of Edge struct to Graph
    pub fn new(edge_lst: &Vec<Edge>) -> Graph {
        let mut graph_hashmap: HashMap<usize, Vec<Edge>> = HashMap::new();
        for edge in edge_lst.iter() {
            let source_node: usize = edge.from;
            // Add the Edge to the vector of source node if source node already exists, 
            graph_hashmap.entry(source_node).or_insert(Vec::new()).push(edge.clone());
        }
        return Graph{ content: graph_hashmap };
    }

    // simple getter method that fetch the related content of that ndoe.
    pub fn get_edges(&self, node: usize) -> (usize, Vec<Edge>) {
        return (node, self.content.get(&node).unwrap().clone());
    }

    // method that get the list of neighbors of a node.
    pub fn get_neighbors(&self, node: usize) -> NodeNeighbors {
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



    // This function calculates the clustering coefficient of a given node.
    // Clustering coefficient "is a measure of the degree to which nodes 
    // in a graph tend to cluster together." - Wikipedia.
    // Its formula is cc(n) = 2 * total edges between neighbors of n / # neighbor * (# neighbor - 1)
    pub fn clustering_coefficient(&self, node: usize) -> f64 {
        let neighbors = self.get_neighbors(node);

        let mut edges_btw_nb = 0;
        let mut outdegree_nb_set: HashSet<usize> = HashSet::new();
        let mut indegree_nb_set: HashSet<usize> = HashSet::new();

        for nb in neighbors.output_nodes.iter() {
            outdegree_nb_set.insert(nb.clone());
        }
        for nb in neighbors.input_nodes.iter() {
            indegree_nb_set.insert(nb.clone());
        }

        // total edges between neighbors of n
        for neighbor in neighbors.output_nodes {
            if let Some(edges) = self.content.get(&neighbor) {
                for edge in edges {
                    if outdegree_nb_set.contains(&edge.to) {
                        edges_btw_nb += 1;
                    }
                }
            }
        }

        for neighbor in neighbors.input_nodes {
            if let Some(edges) = self.content.get(&neighbor) {
                for edge in edges {
                    if indegree_nb_set.contains(&edge.to) {
                        edges_btw_nb += 1;
                    }
                }
            }
        }

        let num_nb = indegree_nb_set.len() + outdegree_nb_set.len();
        if num_nb < 2 {
            return 0.0;
        }

        let possible_connections = num_nb * (num_nb - 1);

        return edges_btw_nb as f64 / possible_connections as f64
    }

}
