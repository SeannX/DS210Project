mod graph;
mod data_loader;
use data_loader::read_csv;

fn main() {
    let csv_path = "soc-sign-bitcoinalpha.csv";
    // The list of edges corresponds to the data
    let edge_lst: Vec<graph::Edge> = read_csv(csv_path);

    // Make a more convinient graph using the list of edges.
    let graph: graph::Graph = graph::mk_graph(&edge_lst);
    
}
