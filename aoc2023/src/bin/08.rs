use hashbrown::HashMap;
use num::integer::lcm;
use regex::Regex;

#[aors::main]
fn main(input: &str) -> (usize, usize) {
    let d: Vec<char> = input.lines().next().unwrap().chars().collect();
    let re = Regex::new(r"([A-Z0-9]+) = \(([A-Z0-9]+), ([A-Z0-9]+)\)").unwrap();
    let m: HashMap<&str, (&str, &str)> = re
        .captures_iter(input)
        .filter_map(|m| Some((m.get(1)?.as_str(), (m.get(2)?.as_str(), m.get(3)?.as_str()))))
        .collect();

    let s = m.keys().filter(|k| k.ends_with("A")).map(|k| *k).collect();
    let e = m.keys().filter(|k| k.ends_with("Z")).map(|k| *k).collect();
    (run(vec!["AAA"], vec!["ZZZ"], &m, &d), run(s, e, &m, &d))
}

fn run(s: Vec<&str>, e: Vec<&str>, m: &HashMap<&str, (&str, &str)>, d: &Vec<char>) -> usize {
    s.iter().fold(1, |acc, s| lcm(acc, cycle(s, &e, &m, &d)))
}

fn cycle(s: &str, e: &Vec<&str>, m: &HashMap<&str, (&str, &str)>, d: &Vec<char>) -> usize {
    let (mut s, mut i) = (s, 0);
    while !e.contains(&s) {
        s = match d[i % d.len()] {
            'L' => m.get(&s).unwrap().0,
            _ => m.get(&s).unwrap().1,
        };
        i += 1;
    }
    i
}
