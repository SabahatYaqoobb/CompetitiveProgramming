use std::collections::{VecDeque, HashMap};

fn is_bipartite(graph: &Vec<Vec<i32>>) -> bool {
    let n_nodes = graph.len();
    let mut color_arr: HashMap<usize, i32> = HashMap::new();

    for start_node in 0..n_nodes {
        if !color_arr.contains_key(&start_node) {
            if !bipartite_bfs(&graph, start_node, &mut color_arr) {
                return false;
            }
        }
    }

    true
}

// returns false if a node and a neighbor have the same color
fn bipartite_bfs(graph: &Vec<Vec<i32>>, start: usize, color_arr: &mut HashMap<usize, i32>) -> bool {
    let mut queue = VecDeque::new();
    queue.push_back(start);
    color_arr.insert(start, 1); // start node gets color 1

    while let Some(node) = queue.pop_front() {
        for (neighbor, &is_edge) in graph[node].iter().enumerate() {
            if is_edge == 1 {
                if let Some(color) = color_arr.get(&neighbor) {
                    if *color == color_arr[&node] {
                        return false; // Same color found, not bipartite
                    }
                } else {
                    color_arr.insert(neighbor, 1 - color_arr[&node]); // Assign alternate color
                    queue.push_back(neighbor);
                }
            }
        }
    }

    true
}

fn main() {
    let graph1 = vec![
        vec![0, 1, 0],
        vec![1, 0, 1],
        vec![0, 1, 0],
    ];
    assert!(is_bipartite(&graph1));

    let graph2 = vec![
        vec![0, 0, 1, 1],
        vec![0, 0, 0, 1],
        vec![1, 0, 0, 1],
        vec![1, 1, 1, 0],
    ];
    assert!(!is_bipartite(&graph2));
}
