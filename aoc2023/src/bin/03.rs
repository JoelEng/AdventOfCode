use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

#[aors::main]
fn main(input: &str) -> (u32, u32) {
    let number_regex = Regex::new("[0-9]+").unwrap();
    let mut pos_map = HashMap::new();
    let mut map = HashMap::new();

    for n in number_regex.captures_iter(input) {
        let n = n.get(0).unwrap();
        let s = n.as_str().parse::<u32>().unwrap();
        map.insert(n.start(), s);
        for pos in n.start()..n.end() {
            pos_map.insert(pos, n.start());
        }
    }

    let p1 = a(input, &pos_map, &map, false).iter().flatten().sum();
    let p2 = a(input, &pos_map, &map, true);
    (p1, p2.iter().map(|v| v.iter().product::<u32>()).sum())
}

fn a(
    input: &str,
    pos_map: &HashMap<usize, usize>,
    map: &HashMap<usize, u32>,
    p2: bool,
) -> Vec<Vec<u32>> {
    let mut res = vec![];
    let symbol_regex = Regex::new(r"[^0-9.\n]").unwrap();

    for s in symbol_regex.captures_iter(input) {
        let s = s.get(0).unwrap();
        let mut set = HashSet::new();
        for pos in adjacent(s.start(), input) {
            if let Some(pos) = pos_map.get(&pos) {
                set.insert(*pos);
            }
        }
        if !p2 || set.len() == 2 {
            res.push(set.iter().map(|pos| *map.get(pos).unwrap()).collect());
        }
    }
    res
}

fn adjacent(pos: usize, input: &str) -> Vec<usize> {
    let width = input.lines().next().unwrap().len() as isize + 1;
    let x = pos as isize % width;
    let y = pos as isize / width;
    let len = input.len() as isize;
    let mut v = vec![];
    for i in -1..=1 {
        for j in -1..=1 {
            let x = x + j;
            let y = y + i;
            if (0..=len / width).contains(&y) && (0..=width).contains(&x) {
                v.push((pos as isize + i * width + j) as usize);
            }
        }
    }
    v
}
