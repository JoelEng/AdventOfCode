use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[aors::main]
fn main(input: &str) -> (usize, usize) {
    let graph = get_input(input);

    (p1(&graph), p2(&graph))
}

fn p1(graph: &HashMap<String, Vec<String>>) -> usize {
    routes_through(
        &"start".to_string(),
        false,
        &vec!["start".to_string()],
        graph,
        "start".to_string(),
    )
    .len()
}

fn p2(graph: &HashMap<String, Vec<String>>) -> usize {
    routes_through(
        &"start".to_string(),
        true,
        &vec!["start".to_string()],
        &graph,
        "start".to_string(),
    )
    .len()
}

fn get_input(input: &str) -> HashMap<String, Vec<String>> {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    for line in input.lines() {
        let (a, b) = line.split('-').next_tuple().unwrap();
        let (a, b) = (a.to_string(), b.to_string());
        graph.entry(a.clone()).or_insert(vec![]).push(b.clone());
        graph.entry(b).or_insert(vec![]).push(a);
    }
    graph
}

/// Returns the number of possible routes through this cave
fn routes_through(
    cave: &String,
    can_revisit_small: bool,
    can_not_visit: &Vec<String>,
    graph: &HashMap<String, Vec<String>>,
    path: String,
) -> HashSet<String> {
    if cave == "end" {
        let mut set = HashSet::new();
        set.insert(path);
        set
    } else {
        let mut paths = HashSet::new();
        let mut new_can_not_visit = can_not_visit.clone();
        if *cave == cave.to_lowercase() {
            new_can_not_visit.push(cave.clone());
        }
        for neighbour in graph
            .get(cave)
            .unwrap()
            .iter()
            .filter(|n| !can_not_visit.contains(n))
        {
            if can_revisit_small
                && *cave == cave.to_lowercase()
                && cave != "start"
                && neighbour != "end"
            {
                paths.extend(routes_through(
                    neighbour,
                    false,
                    &can_not_visit,
                    graph,
                    format!("{} {}", path, neighbour),
                ));
            }
            paths.extend(routes_through(
                neighbour,
                can_revisit_small,
                &new_can_not_visit,
                graph,
                format!("{} {}", path, neighbour),
            ));
        }
        paths
    }
}
