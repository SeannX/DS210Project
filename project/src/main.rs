mod graph;
use graph::Graph;
mod data_loader;
use data_loader::read_csv;
mod analyze;
use analyze::GraphInfo;

fn main() {
    let csv_path = "soc-sign-bitcoinalpha.csv";
    
    println!("\n------------- Genral Info -------------");
    // The list of edges corresponds to the data
    let edge_lst: Vec<graph::Edge> = read_csv(csv_path);

    // Make a more convinient graph using the list of edges.
    let graph: Graph = Graph::new(&edge_lst);

    let graph_info: GraphInfo = GraphInfo::get_info(&graph.clone());

    let num_nodes: usize = graph_info.graph.content.len();
    println!("Number of nodes in this data: {}\n", num_nodes);

    let num_sub_graphs: usize = graph_info.sub_graphs.len();
    println!("Number of subgraphs in this data: {}", num_sub_graphs);

    let mut graph_index = 1;

    for sub_graph in graph_info.sub_graphs.iter() {
        println!("Number of nodes in sub graph {}: {}", graph_index, sub_graph.content.len());
        graph_index += 1;
    }

    let clustering_centrality_result: String = graph_info.clone().analyze_clustering_centrality(2.0, -2.0);

    let k_representatives_result: String = graph_info.clone().find_k_representatives(15);

    println!("\n------------- Clustering and Centrality of nodes with high / low trust score -------------");

    println!("{}", clustering_centrality_result);

    println!("\n------------- K representatives -------------");

    println!("{}", k_representatives_result);
}
