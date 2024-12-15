use crate::graph::Graph;
use crate::graph::Edge;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct GraphInfo {
    pub graph: Graph,
    pub nodes_indegree: HashMap<usize, f64>,
    pub nodes_outdegree: HashMap<usize, f64>,
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
    
        let mean_indegree: f64 = self.nodes_outdegree.values().cloned().sum::<f64>()
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
                if degree as f64 >= mean_indegree {
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
                if degree as f64 >= mean_indegree {
                    high_centrality_count_low_score += 1;
                }
            }
        }
        
        
        let high_trust_count = high_trust_nodes.len();
        let mut clustering_percentage_high_score = 0.0;
        let mut centrality_percentage_high_score = 0.0;

        if high_trust_count != 0 {
            clustering_percentage_high_score = (high_clustering_count_high_score as f64 / high_trust_count as f64) * 100.0;
            centrality_percentage_high_score = (high_centrality_count_high_score as f64 / high_trust_count as f64) * 100.0;
        }

        let low_trust_count = low_trust_nodes.len();
        let mut clustering_percentage_low_score = 0.0;
        let mut centrality_percentage_low_score = 0.0;

        if low_trust_count != 0 {
            clustering_percentage_low_score = (high_clustering_count_low_score as f64 / low_trust_count as f64) * 100.0;
            centrality_percentage_low_score = (high_centrality_count_low_score as f64 / low_trust_count as f64) * 100.0;
        }


    
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

    pub fn find_k_representatives(&self, k: usize) -> String {
        // normalize the data: centrality (# indegree), trust_scores, clustering coefficients.
        let (min_indegree, max_indegree) = Self::find_min_max(&self.nodes_indegree);
        let (min_clustering, max_clustering) = Self::find_min_max(&self.clustering_coefficients);
    
        let mut node_scores: Vec<(usize, f64, f64)> = self.graph.content.keys().map(|&node| {
            let normalized_centrality = Self::normalize(*self.nodes_indegree.get(&node).unwrap_or(&0.0) as f64 as f64, min_indegree, max_indegree);
            let normalized_clustering =  Self::normalize(*self.clustering_coefficients.get(&node).unwrap_or(&0.0) as f64, min_clustering, max_clustering);
            let score = 0.7 * normalized_centrality + 0.3 * normalized_clustering;
            let trust_score = *self.trust_scores.get(&node).unwrap_or(&0.0);
            (node, score, trust_score)
        }).collect();
    
        // Sort base on score then by trust score if equal to remain order.
        node_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap().then(b.2.partial_cmp(&a.2).unwrap()));
        
    
        // get the k representative 
        let mut representatives = Vec::new();
        for i in 0..k {
            if i < node_scores.len() {
                representatives.push(node_scores[i].0);
            }
        }

        let mut clustering_sum: f64 = 0.0;
        for repr in representatives.iter() {
            clustering_sum += self.clustering_coefficients.get(&repr).unwrap_or(&0.0);
        }

        let avg_representative_clustering: f64 = clustering_sum / representatives.len() as f64;
    
        // Ratio of num representative to total num of nodes
        let representative_ratio = (representatives.len() as f64 / self.graph.content.len() as f64) * 100.0;
    
        let mut avg_trust_score = Vec::new();
    
        for node in representatives.iter() {
            avg_trust_score.push(self.trust_scores.get(node).unwrap());
        }
    
        // Prepare the result message
        let satisfaction_message = format!(
            "\nSelected Representatives: {:?}
            \nThe number of representatives is {:.2}% of total nodes.
            \nAverage trust scores of each representative: {:?}
            \nAverage trust scores of all representatives: {}
            \nAverage clustering coefficient of representatives: {}",
            representatives,
            representative_ratio,
            avg_trust_score,
            avg_trust_score.clone().into_iter().sum::<f64>() / avg_trust_score.len() as f64,
            avg_representative_clustering,
        );
    
        return satisfaction_message;
    }

    // Helper function to find the min and max values of a HashMap's values
    fn find_min_max(values: &HashMap<usize, f64>) -> (f64, f64) {
        let (mut min_value, mut max_value) = (f64::MAX, f64::MIN);

        for &value in values.values() {
            if value < min_value {
                min_value = value;
            }
            if value > max_value {
                max_value = value;
            }
        }

        if min_value == f64::MAX {
            min_value = 0.0; // Handle empty case
        }
        if max_value == f64::MIN {
            max_value = 0.0; // Handle empty case
        }

        (min_value, max_value)
    }

    /// Helper function to normalize a value between min and max
    fn normalize(value: f64, min: f64, max: f64) -> f64 {
        if max - min == 0.0 {
            0.0
        } else {
            (value - min) / (max - min)
        }
    }
}  

// ----------------------- TESTS ----------------------- 
#[test]
fn test_get_info_basic() {
    let edges = vec![
        Edge { from: 1, to: 2, weight: 1.0 },
        Edge { from: 2, to: 3, weight: 2.0 },
        Edge { from: 3, to: 1, weight: 3.0 },
    ];
    let graph = Graph::new(&edges);
    let info = GraphInfo::get_info(&graph);

    assert_eq!(info.graph.content.len(), 3); // Should be three nodes
    assert!(info.nodes_indegree.contains_key(&1)); // Should contain node 1
    assert!(info.nodes_outdegree.contains_key(&1)); // Should contain node 1
    assert!(info.nodes_indegree.get(&1).unwrap() - 1.0 < 1e-8); // Indegre should be 1
    assert!(info.nodes_outdegree.get(&1).unwrap() - 1.0 < 1e-8); // outdegree should be 1
}

#[test]
fn test_get_info_empty_graph() {
    let edges: Vec<Edge> = vec![];
    let graph = Graph::new(&edges);
    let info = GraphInfo::get_info(&graph);

    assert_eq!(info.graph.content.len(), 0); // No nodes
    assert!(info.nodes_indegree.is_empty()); // No indegrees
    assert!(info.nodes_outdegree.is_empty()); // No outdegrees
    assert!(info.clustering_coefficients.is_empty()); // No clustering coefficients
    assert!(info.sub_graphs.is_empty()); // No subgraphs
}

#[test]
fn test_analyze_clustering_centrality() {
    let edges = vec![
        Edge { from: 1, to: 2, weight: 1.0 },
        Edge { from: 2, to: 3, weight: 2.0 },
    ];
    let graph = Graph::new(&edges);
    let info = GraphInfo::get_info(&graph);

    let result = info.analyze_clustering_centrality(1.0, 0.0);

    println!("{}", result);
    assert!(result.contains("Nodes with trust score >= 1: 2 nodes."));
    assert!(result.contains("Percentage with high clustering: 0.00"));
}

#[test]
fn test_analyze_clustering_centrality_no_scores() {
    let edges = vec![
        Edge { from: 1, to: 2, weight: 0.5 },
    ];
    let graph = Graph::new(&edges);
    let info = GraphInfo::get_info(&graph);

    let result = info.analyze_clustering_centrality(2.0, -1.0);
    assert!(result.contains("Nodes with trust score >= 2: 0 nodes.")); // No nodes match trust scores
    assert!(result.contains("Percentage with high clustering: 0.00%.")); // should be 0.0%
}

#[test]
fn test_find_k_representatives() {
    let edges = vec![
        Edge { from: 1, to: 2, weight: 1.0 },
        Edge { from: 2, to: 3, weight: 2.0 },
        Edge { from: 3, to: 1, weight: 3.0 },
    ];
    let graph = Graph::new(&edges);
    let info = GraphInfo::get_info(&graph);

    let result = info.find_k_representatives(2);
    assert!(result.contains("Selected Representatives"));
    assert!(result.contains("Average clustering coefficient of representatives"));
}

// Test request more representatives than nodes
#[test]
fn test_find_k_representatives_insufficient_nodes() {
    let edges = vec![
        Edge { from: 1, to: 2, weight: 1.0 },
    ];
    let graph = Graph::new(&edges);
    let info = GraphInfo::get_info(&graph);

    let result = info.find_k_representatives(5);
    assert!(result.contains("Selected Representatives: [1, 2]") || result.contains("Selected Representatives: [2, 1]"));
}

#[test]
fn test_find_min_max_typical() {
    use std::collections::HashMap;

    let mut values = HashMap::new();
    values.insert(1, 10.0);
    values.insert(2, 20.0);
    values.insert(3, 5.0);
    values.insert(4, 15.0);

    let (min, max) = GraphInfo::find_min_max(&values);

    assert_eq!(min, 5.0);
    assert_eq!(max, 20.0);
}


#[test]
fn test_find_min_max_empty() {
    use std::collections::HashMap;

    let values: HashMap<usize, f64> = HashMap::new();

    let (min, max) = GraphInfo::find_min_max(&values);

    assert_eq!(min, 0.0); // Should return 0.0
    assert_eq!(max, 0.0); // Same
}

#[test]
fn test_normalize_typical() {
    let value = 15.0;
    let min = 10.0;
    let max = 20.0;

    let normalized = GraphInfo::normalize(value, min, max);

    assert_eq!(normalized, 0.5); // (15 - 10) / (20 - 10) = 0.5
}

#[test]
fn test_normalize_min_equals_max() {
    let value = 10.0;
    let min = 10.0;
    let max = 10.0;

    let normalized = GraphInfo::normalize(value, min, max);

    assert_eq!(normalized, 0.0); // Should handle division by zero.
}
