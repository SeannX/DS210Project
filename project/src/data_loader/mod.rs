use csv::ReaderBuilder;
use std::fs::File;
use std::collections::HashSet;

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

// ----------------------- TESTS ----------------------- 

#[test]
fn test_read_csv_num_edges() {
    let file = "soc-sign-bitcoinalpha.csv";
    let edges = read_csv(file);

    // Num edges should = 24186 according to data base's documentation
    assert_eq!(edges.len(), 24186, "The number of edges should be 24186");
}

#[test]
fn test_read_csv_num_nodes() {
    let file = "soc-sign-bitcoinalpha.csv";
    let edges = read_csv(file);

    let mut nodes = HashSet::new();
    for edge in &edges {
        nodes.insert(edge.from);
        nodes.insert(edge.to);
    }

    // Num nodes should = 3783 according to data base's documentation.
    assert_eq!(nodes.len(), 3783, "The number of nodes should be 3783");

    println!("Test passed: Correct number of edges and nodes detected.");
}
