mod data_loader;
use data_loader::Edge;
use data_loader::read_csv;

fn main() {
    let csv_path = "soc-sign-bitcoinalpha.csv";
    let graph: Vec<Edge> = read_csv(csv_path);
    println!("{:?}", graph);
}
