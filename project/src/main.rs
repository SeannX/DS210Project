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

    let graph_info: GraphInfo = GraphInfo::get_info(&graph.clone());

    let result: String = graph_info.clone().whole_graph_analyze(2.0);


    println!("{:?}", result);

    println!("Num nodes total: {:?}", graph.clone().content.len());

    /*
    for s in result2.iter() {
        println!("{:?}\n", s);
    }

    println!("{:?}\n", result3);
*/
}
