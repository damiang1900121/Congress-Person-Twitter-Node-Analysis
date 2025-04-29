use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use rand::seq::SliceRandom;

#[derive(Debug)]
struct Graph {
    adj_list: HashMap<usize, Vec<usize>>,
}

impl Graph {
    fn new() -> Self {
        Self {
            adj_list: HashMap::new(),
        }
    }

    // Adds an edge (only unweighted)
    fn add_edge(&mut self, source: usize, target: usize) {
        self.adj_list.entry(source).or_insert_with(Vec::new).push(target);
    }

    // Load from edgelist: ignore weights
    fn from_edgelist(path: &str) -> Self {
        let file = File::open(path).expect("Unable to open edgelist file");
        let reader = BufReader::new(file);

        let mut graph = Graph::new();

        for line in reader.lines() {
            let line = line.expect("Unable to read line");
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 2 {
                continue;
            }
            let source: usize = parts[0].parse().unwrap();
            let target: usize = parts[1].parse().unwrap();
            graph.add_edge(source, target);
        }

        graph
    }


    // BFS: find shortest path lengths from start
    fn bfs(&self, start: usize) -> HashMap<usize, usize> {
        let mut distances = HashMap::new();
        let mut queue = VecDeque::new();

        distances.insert(start, 0);
        queue.push_back(start);

        while let Some(node) = queue.pop_front() {
            if let Some(neighbors) = self.adj_list.get(&node) {
                for &neighbor in neighbors {
                    if !distances.contains_key(&neighbor) {
                        distances.insert(neighbor, distances[&node] + 1);
                        queue.push_back(neighbor);
                    }
                }
            }
        }

        distances
    }
}

// Generate random graph
fn generate_random_graph(num_nodes: usize, num_edges: usize) -> Graph {
    let mut graph = Graph::new();
    let mut rng = rand::thread_rng();
    let nodes: Vec<usize> = (0..num_nodes).collect();

    for _ in 0..num_edges {
        let source = *nodes.choose(&mut rng).unwrap();
        let target = *nodes.choose(&mut rng).unwrap();
        if source != target {
            graph.add_edge(source, target);
        }
    }

    graph
}

// Average shortest path length
fn average_path_length(graph: &Graph) -> f64 {
    let nodes: Vec<usize> = graph.adj_list.keys().copied().collect();
    let mut total_distance = 0;
    let mut path_count = 0;

    for &node in &nodes {
        let distances = graph.bfs(node);
        for &d in distances.values() {
            if d > 0 {
                total_distance += d;
                path_count += 1;
            }
        }
    }

    if path_count == 0 {
        0.0
    } else {
        total_distance as f64 / path_count as f64
    }
}

fn main() {
    println!("Loading Congress graph...");
    let congress_graph = Graph::from_edgelist("congress.edgelist");

    println!("Generating random graph...");
    let random_graph = generate_random_graph(475, 13289);

    println!("Calculating average path lengths...");
    let avg_congress = average_path_length(&congress_graph);
    let avg_random = average_path_length(&random_graph);

    println!("Average path length (Congress): {:.4}", avg_congress);
    println!("Average path length (Random): {:.4}", avg_random);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_edge() {
        let mut graph = Graph::new();
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);

        assert_eq!(graph.adj_list.get(&1).unwrap(), &vec![2, 3]);
    }

    #[test]
    fn test_bfs_simple() {
        let mut graph = Graph::new();
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);

        let distances = graph.bfs(0);
        
        assert_eq!(distances.get(&0), Some(&0));
        assert_eq!(distances.get(&1), Some(&1));
        assert_eq!(distances.get(&2), Some(&2));
        assert_eq!(distances.get(&3), Some(&3));
    }

    #[test]
    fn test_average_path_length_small_graph() {
        let mut graph = Graph::new();
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let avg = average_path_length(&graph);

        let expected_avg = (1 + 2 + 1) as f64 / 3.0;

        assert!((avg - expected_avg).abs() == 0.0);
    }
}