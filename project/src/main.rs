mod graph;
use graph::Graph;
mod data_loader;
use data_loader::read_csv;
mod analyze;
use analyze::GraphInfo;

fn main() {
    let csv_path = "soc-sign-bitcoinalpha.csv";
    // The list of edges corresponds to the data
    let edge_lst: Vec<graph::Edge> = read_csv(csv_path);

    // Make a more convinient graph using the list of edges.
    let graph: Graph = Graph::new(&edge_lst);

    let graph_info: GraphInfo = GraphInfo::get_info(&graph);
    
    println!("{:?}", graph_info.analyze_network_spread());

    println!("{:?}", graph_info.analyze_trust_clustering());
}
