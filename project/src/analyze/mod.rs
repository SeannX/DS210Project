use crate::graph::Graph;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct GraphInfo {
    graph: Graph,
    nodes_indegree: HashMap<usize, usize>,
    nodes_outdegree: HashMap<usize, usize>,
    clustering_coefficients: HashMap<usize, f64>,
    sub_graphs: Vec<Graph>,
    trust_scores: HashMap<usize, f64>,
}

impl GraphInfo {
    // Constructor to get all the graph info of the data using
    // methods from Graph.
    pub fn get_info(graph: &Graph) -> GraphInfo {
        // Indegrees and outdegrees
        let (nodes_indegree, nodes_outdegree) = graph.get_degrees();

        // Clustering coefficients
        let mut clustering_coefficients = HashMap::new();
        for &node in graph.content.keys() {
            clustering_coefficients.insert(node, graph.clustering_coefficient(node));
        }

        // Clustering coefficients
        let mut trust_scores = HashMap::new();
        for &node in graph.content.keys() {
            clustering_coefficients.insert(node, graph.get_trust_score(node));
        }

        // Subgraphs
        let sub_graphs = graph.find_subgraphs();

        GraphInfo {
            graph: graph.clone(),
            nodes_indegree: nodes_indegree,
            nodes_outdegree: nodes_outdegree,
            clustering_coefficients: clustering_coefficients,
            sub_graphs: sub_graphs,
            trust_scores: trust_scores,
        }
    }

    // Determines whether the network has major transaction nets or is separate
    pub fn analyze_network_spread(&self) -> String {
        let subgraph_count = self.sub_graphs.len();
        let mut largest_subgraph = None;
        let mut largest_size = 0;

        for subgraph in &self.sub_graphs {
            let subgraph_size = subgraph.content.len();
            if subgraph_size > largest_size {
                largest_size = subgraph_size;
                largest_subgraph = Some(subgraph);
            }
        }

        let spread_message = if subgraph_count == 1 {
            "The network forms a single major transaction net.".to_string()
        } else {
            format!("The network is separated into {} distinct transaction nets.", subgraph_count)
        };

        if let Some(largest) = largest_subgraph {
            let nodes: Vec<_> = largest.content.keys().cloned().collect();
            format!("{} The largest subgraph has {} nodes.", spread_message, largest_size)
        } else {
            spread_message
        }
    }

}