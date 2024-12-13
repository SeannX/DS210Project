use crate::graph::Graph;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct GraphInfo {
    pub graph: Graph,
    pub nodes_indegree: HashMap<usize, usize>,
    pub nodes_outdegree: HashMap<usize, usize>,
    pub clustering_coefficients: HashMap<usize, f64>,
    pub sub_graphs: Vec<Graph>,
    pub trust_scores: HashMap<usize, f64>,
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
            trust_scores.insert(node, graph.get_trust_score(node));
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

    pub fn analyze_graph(&self, graph: &Graph, high_score: usize) -> String {
        // Filter nodes with trust score > 2.0
        let mut high_trust_nodes = vec![];
        for &node in graph.content.keys() {
            if let Some(&trust_score) = self.trust_scores.get(&node) {
                if trust_score < high_score {
                    high_trust_nodes.push(node);
                }
            }
        }
        // In case if there's no nodes with the high_score specified
        if high_trust_nodes.is_empty() {
            return "No high trust nodes in the graph.".to_string();
        }
    
        let avg_clustering: f64 = self.clustering_coefficients.values().sum::<f64>()
            / self.clustering_coefficients.len() as f64;
    
        let max_outdegree = self.nodes_outdegree.values().cloned().max().unwrap_or(0) as f64;
        let mean_outdegree: f64 = self.nodes_outdegree.values().cloned().sum::<usize>() as f64
            / self.nodes_outdegree.len() as f64;
    
        let mut high_clustering_count = 0;
        let mut high_centrality_count = 0;
    
        for &node in &high_trust_nodes {
            // Check cc to see if it is well-clusterred.
            let clustering = self.clustering_coefficients.get(&node).unwrap_or(&0.0);
            if *clustering > avg_clustering {
                high_clustering_count += 1;
            }
    
            // Calculate centrality
            if let Some(&degree) = self.nodes_outdegree.get(&node) {
                if degree as f64 >= 0.75 * max_outdegree || degree as f64 >= mean_outdegree {
                    high_centrality_count += 1;
                }
            }
        }
    
        let high_trust_count = high_trust_nodes.len();
        let clustering_percentage = (high_clustering_count as f64 / high_trust_count as f64) * 100.0;
        let centrality_percentage = (high_centrality_count as f64 / high_trust_count as f64) * 100.0;
    
        // Return the result
        format!(
            "Graph: {} nodes.\nHigh trust nodes: {} ({} nodes).\nPercentage with high clustering: {:.2}%.\nPercentage with high centrality: {:.2}%.",
            graph.content.len(), high_trust_count, high_trust_nodes.len(), clustering_percentage, centrality_percentage )
    }

    /// Analyzes the largest subgraph
    pub fn get_largest_subgraph_analyze(&self) -> String {
        let largest_subgraph = self.sub_graphs.iter().max_by_key(|subgraph| subgraph.content.len());
        if let Some(subgraph) = largest_subgraph {
            self.analyze_graph(subgraph)
        } else {
            "No subgraphs available.".to_string()
        }
    }

    /// Analyzes individual subgraphs
    pub fn get_individual_subgraph_analyze(&self) -> Vec<String> {
        self.sub_graphs.iter().enumerate().map(|(i, subgraph)| {
            format!("Subgraph {} Analysis:\n{}", i + 1, self.analyze_graph(subgraph))
        }).collect()
    }

    /// Analyzes the entire graph
    pub fn whole_graph_analyze(&self) -> String {
        self.analyze_graph(&self.graph)
    }

}  