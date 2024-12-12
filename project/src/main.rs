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

    let result1: String = graph_info.clone().get_largest_subgraph_analyze();

    let result2: Vec<String> = graph_info.clone().get_individual_subgraph_analyze();

    let result3: String = graph_info.clone().whole_graph_analyze();

    let ts_lst = graph_info.clone().trust_scores;

    let total_ts: f64 = ts_lst.values().sum::<f64>();

    let mut avg: f64 = total_ts / ts_lst.len() as f64;

    for s in graph_info.clone().sub_graphs.iter() {
        println!("Graph: {:?}\n", s.content);
    }

    println!("Num nodes total: {:?}", graph.clone().content.len());

    println!("{:?}\n", result1);

    /*
    for s in result2.iter() {
        println!("{:?}\n", s);
    }

    println!("{:?}\n", result3);
*/
}
