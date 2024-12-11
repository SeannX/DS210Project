use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

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
            let source_node: usize = edge.from;
            // Add the Edge to the vector of source node if source node already exists, 
            graph_hashmap.entry(source_node).or_insert(Vec::new()).push(edge.clone());
        }
        return Graph{ content: graph_hashmap };
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


    // compute the indegree and outdegree of ALL nodes in the graph.
    // return value: (indegree HashMap, outdegre HashMap)
    pub fn get_degrees(&self) -> (HashMap<usize, usize>, HashMap<usize, usize>) {
        let mut in_degree = HashMap::new();
        let mut out_degree = HashMap::new();

        for (node, edges) in &self.content {
            out_degree.insert(*node, edges.len());

            for edge in edges {
                *in_degree.entry(edge.to).or_insert(0) += 1;
            }
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

    pub fn find_subgraphs(&self) -> Vec<Graph> {
        let mut visited = HashSet::new();
        let mut subgraphs = Vec::new();

        for &node in self.content.keys() {
            if !visited.contains(&node) {
                // Use BFS to find all nodes in the connected subgraph
                let subgraph_nodes = self.bfs(node);

                // Construct the subgraph from the collected nodes
                let mut subgraph_content = HashMap::new();
                for node in subgraph_nodes {
                    if let Some(edges) = self.content.get(&node) {
                        subgraph_content.insert(node, edges.clone());
                    }
                    visited.insert(node);
                }

                subgraphs.push(Graph { content: subgraph_content });
            }
        }

        return subgraphs;
    }

    // BFS algorithm that find all the connected node
    // a helper function of find_subgraphs
    pub fn bfs(&self, start_node: usize) -> HashSet<usize> {
        let mut sub_graph_nodes = HashSet::new();
        let mut queue = VecDeque::new();
        let mut visited: HashSet<usize> = HashSet::new();

        queue.push_back(start_node); 
        sub_graph_nodes.insert(start_node);

        while let Some(node) = queue.pop_front() {
            sub_graph_nodes.insert(node);

            if let Some(edges) = self.content.get(&node) {
                for edge in edges {
                    if !visited.contains(&edge.to) {
                        visited.insert(edge.to);
                        queue.push_back(edge.to);
                    }
                }
            }
        }
        return sub_graph_nodes;
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
