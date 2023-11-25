use petgraph::{
    algo::dijkstra,
    graph::{Graph, NodeIndex},
};

#[aors::main]
fn main(input: &str) -> (u64, u64) {
    (lowest_risk_path(1, input), lowest_risk_path(5, input))
}

fn lowest_risk_path(size: u64, input: &str) -> u64 {
    let (g, nodes) = create_graph(get_input(input, size));
    let path = dijkstra(
        &g,
        nodes[0][0],
        Some(nodes[nodes.len() - 1][nodes[0].len() - 1]),
        |e| *e.weight(),
    );
    path[&nodes[nodes.len() - 1][nodes[0].len() - 1]]
}

fn get_input(input: &str, size: u64) -> Vec<Vec<u64>> {
    let mut v: Vec<Vec<u64>> = Vec::new();
    for line in input.lines() {
        let mut elems: Vec<u64> = Vec::new();
        for i in 0..size {
            let mut values: Vec<u64> = line
                .chars()
                .map(|c| {
                    let n = c.to_digit(10).unwrap() as u64;
                    (n + i) % 10 + (n + i) / 10
                })
                .collect();
            elems.append(&mut values);
        }
        v.push(elems);
    }
    let mut matrix: Vec<Vec<u64>> = Vec::new();
    for i in 0..size {
        for line in &v {
            let values: Vec<u64> = line.iter().map(|c| (c + i) % 10 + (c + i) / 10).collect();
            matrix.push(values);
        }
    }
    matrix
}

fn create_graph(matrix: Vec<Vec<u64>>) -> (Graph<(), u64>, Vec<Vec<NodeIndex>>) {
    let mut g = Graph::<(), u64>::new();
    let mut nodes: Vec<Vec<NodeIndex>> =
        vec![vec![NodeIndex::new(0); matrix[0].len()]; matrix.len()];

    nodes[0][0] = g.add_node(());
    for row in 0..matrix.len() {
        for col in 0..matrix[row].len() {
            let node = nodes[row][col];
            let node_weight = matrix[row][col];
            if col < matrix[0].len() - 1 {
                let move_right = matrix[row][col + 1];
                if nodes[row][col + 1] == 0.into() {
                    nodes[row][col + 1] = g.add_node(());
                }
                g.add_edge(node, nodes[row][col + 1], move_right);
                g.add_edge(nodes[row][col + 1], node, node_weight);
            }
            if row < matrix.len() - 1 {
                let move_down = matrix[row + 1][col];
                if nodes[row + 1][col] == 0.into() {
                    nodes[row + 1][col] = g.add_node(());
                }
                g.add_edge(node, nodes[row + 1][col], move_down);
                g.add_edge(nodes[row + 1][col], node, node_weight);
            }
        }
    }
    (g, nodes)
}
