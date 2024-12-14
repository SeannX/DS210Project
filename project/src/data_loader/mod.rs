use csv::ReaderBuilder;
use std::fs::File;

use crate::graph::Edge;

// function that read the csv_file and construct the datas in to a vector of Edge structs.
pub fn read_csv(file_path: &str) -> Vec<Edge> {
    // file_path - by default, it should be "soc-sign-bitcoinalpha.csv" if you runs the project
    // at DS210Project/project.

    let file = File::open(file_path).unwrap();
    let mut reader = ReaderBuilder::new().has_headers(false).from_reader(file);

    let mut edges = Vec::new();

    for line in reader.records() {
        let line_record = line.unwrap();

        // Parse each line to the corresponding four fields
        let from: usize = line_record[0].parse().unwrap();    // The node that the edge comes from
        let to: usize = line_record[1].parse().unwrap();      // The node that the edge points to
        let weight: f64 = line_record[2].parse().unwrap();    // weight

        edges.push(Edge {from: from, to: to, weight: weight});
    }

    let num_edges: usize = edges.len();
    println!("\nTotal number of edges: {}\n", num_edges);
    return edges;
}
