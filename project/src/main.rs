mod graph;
use graph::Graph;
mod data_loader;
use data_loader::read_csv;
mod analyze;
use analyze::GraphInfo;

fn main() {
    let csv_path = "soc-sign-bitcoinalpha.csv";
    
    println!("\n------------- General Info -------------");
    // The list of edges corresponds to the data
    let edge_lst: Vec<graph::Edge> = read_csv(csv_path);

    // Make a more convinient graph using the list of edges.
    let graph: Graph = Graph::new(&edge_lst);

    let graph_info: GraphInfo = GraphInfo::get_info(&graph.clone());

    // number of total nodes
    let num_nodes: usize = graph_info.graph.content.len();
    println!("Number of nodes in this data: {}\n", num_nodes);

    // Number of sub graphs
    let num_sub_graphs: usize = graph_info.sub_graphs.len();
    println!("Number of subgraphs in this data: {}\n", num_sub_graphs);

    // Average clustering coefficient
    let clustering_coefficient_sum: f64 = graph_info.clustering_coefficients.values().sum();
    let avg_clustering_coefficient: f64 = clustering_coefficient_sum / num_nodes as f64;
    println!("Average clustering coefficient: {:.5}\n", avg_clustering_coefficient);

    // Average trust score
    let trust_score_sum: f64 = graph_info.trust_scores.values().sum();
    let avg_trust_score: f64 = trust_score_sum / num_nodes as f64;
    println!("Average trust score: {:.5}\n", avg_trust_score);

    let mut graph_index = 1;

    for sub_graph in graph_info.sub_graphs.iter() {
        println!("Number of nodes in sub graph {}: {}", graph_index, sub_graph.content.len());
        graph_index += 1;
    }

    println!("\n------------- Clustering and Centrality of nodes with high / low trust score -------------");

    let clustering_centrality_result: String = graph_info.clone().analyze_clustering_centrality(4.0, -2.0);
    println!("{}", clustering_centrality_result);

    let k: usize = 15;
    println!("\n------------- k representatives (k = {}) -------------", k);
    let k_representatives_result: String = graph_info.clone().find_k_representatives(k);
    println!("{}", k_representatives_result);
}

