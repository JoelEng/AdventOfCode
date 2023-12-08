use hashbrown::HashMap;
use num::integer::lcm;
use regex::Regex;

#[aors::main]
fn main(input: &str) -> (usize, usize) {
    let dirs: Vec<char> = input.lines().next().unwrap().chars().collect();
    let re = Regex::new(r"([A-Z0-9]+) = \(([A-Z0-9]+), ([A-Z0-9]+)\)").unwrap();
    let map: HashMap<String, (String, String)> = re
        .captures_iter(input)
        .filter_map(|m| {
            Some((
                m.get(1)?.as_str().to_string(),
                (
                    m.get(2)?.as_str().to_string(),
                    m.get(3)?.as_str().to_string(),
                ),
            ))
        })
        .collect();

    let p1 = run(
        vec!["AAA".to_string()],
        vec!["ZZZ".to_string()],
        &map,
        &dirs,
    );
    let p2 = run(
        map.clone()
            .into_keys()
            .filter(|k| k.ends_with("A"))
            .collect(),
        map.clone()
            .into_keys()
            .filter(|k| k.ends_with("Z"))
            .collect(),
        &map,
        &dirs,
    );
    (p1, p2)
}

fn run(
    start: Vec<String>,
    end: Vec<String>,
    map: &HashMap<String, (String, String)>,
    dirs: &Vec<char>,
) -> usize {
    let mut a = 1;
    for c in start {
        let cycle_size = cycle(c, &end, &map, &dirs);
        a = lcm(a, cycle_size);
    }
    a
}

fn cycle(
    mut current: String,
    end: &Vec<String>,
    map: &HashMap<String, (String, String)>,
    dirs: &Vec<char>,
) -> usize {
    let mut visited = vec![(0, current.clone())];
    let mut i = 0;
    loop {
        // println!("{}", current);
        current = match dirs[i % dirs.len()] {
            'L' => map.get(&current).unwrap().0.clone(),
            _ => map.get(&current).unwrap().1.clone(),
        };
        if visited.contains(&(i % dirs.len(), current.clone())) {
            break;
        }
        visited.push((i % dirs.len(), current.clone()));
        i += 1;
    }
    visited.len() - i % dirs.len() - 1
}
