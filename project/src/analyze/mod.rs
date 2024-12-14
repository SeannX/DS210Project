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

    pub fn analyze_clustering_centrality(&self, high_score: f64, low_score: f64) -> String {
        // Filter nodes with trust score
        let mut high_trust_nodes = vec![];
        let mut low_trust_nodes = vec![];
        for &node in self.graph.content.keys() {
            if let Some(&trust_score) = self.trust_scores.get(&node) {
                if trust_score >= high_score {
                    high_trust_nodes.push(node);
                }
                if trust_score <= low_score {
                    low_trust_nodes.push(node)
                }
            }
        }
    
        let avg_clustering: f64 = self.clustering_coefficients.values().sum::<f64>()
            / self.clustering_coefficients.len() as f64;
    
        let max_outdegree = self.nodes_outdegree.values().cloned().max().unwrap_or(0) as f64;
        let mean_outdegree: f64 = self.nodes_outdegree.values().cloned().sum::<usize>() as f64
            / self.nodes_outdegree.len() as f64;
    
        let mut high_clustering_count_high_score = 0;
        let mut high_centrality_count_high_score = 0;
        let mut high_clustering_count_low_score = 0;
        let mut high_centrality_count_low_score = 0;
    
        for &node in &high_trust_nodes {
            // Check cc to see if it is well-clusterred.
            let clustering = self.clustering_coefficients.get(&node).unwrap_or(&0.0);
            if *clustering > avg_clustering {
                high_clustering_count_high_score += 1;
            }
    
            // Calculate centrality
            if let Some(&degree) = self.nodes_outdegree.get(&node) {
                if degree as f64 >= mean_outdegree {
                    high_centrality_count_high_score += 1;
                }
            }
        }

        for &node in &low_trust_nodes {
            // Check cc to see if it is well-clusterred.
            let clustering = self.clustering_coefficients.get(&node).unwrap_or(&0.0);
            if *clustering > avg_clustering {
                high_clustering_count_low_score += 1;
            }
    
            // Calculate centrality
            if let Some(&degree) = self.nodes_outdegree.get(&node) {
                if degree as f64 >= mean_outdegree {
                    high_centrality_count_low_score += 1;
                }
            }
        }
    
        let high_trust_count = high_trust_nodes.len();
        let clustering_percentage_high_score = (high_clustering_count_high_score as f64 / high_trust_count as f64) * 100.0;
        let centrality_percentage_high_score = (high_centrality_count_high_score as f64 / high_trust_count as f64) * 100.0;

        let low_trust_count = low_trust_nodes.len();
        let clustering_percentage_low_score = (high_clustering_count_low_score as f64 / low_trust_count as f64) * 100.0;
        let centrality_percentage_low_score = (high_centrality_count_low_score as f64 / low_trust_count as f64) * 100.0;


    
        // Return the result
        return format!("\nNodes with trust score >= {}: {} nodes.
                \nPercentage with high clustering: {:.2}%.
                \nPercentage with high centrality: {:.2}%.
                \nNodes with trust score <= {}: {} nodes.
                \nPercentage with high clustering: {:.2}%.
                \nPercentage with high centrality: {:.2}%.",
                high_score, high_trust_count,
                clustering_percentage_high_score, 
                centrality_percentage_high_score,
                low_score, low_trust_count,
                clustering_percentage_low_score, 
                centrality_percentage_low_score,
                );
    }

}  