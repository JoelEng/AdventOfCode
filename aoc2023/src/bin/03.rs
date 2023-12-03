use regex::Regex;
use std::collections::BTreeMap;
use std::collections::HashSet;

#[aors::main]
fn main(input: &str) -> (u32, u32) {
    let mut map = BTreeMap::new();
    for n in Regex::new("[0-9]+").unwrap().captures_iter(input) {
        let n = n.get(0).unwrap();
        let s = n.as_str().parse::<u32>().unwrap();
        map.insert(n.start(), s);
    }

    let p1 = a(input, &map, false).unwrap().iter().flatten().sum();
    let p2 = a(input, &map, true).unwrap();
    (p1, p2.iter().map(|v| v.iter().product::<u32>()).sum())
}

fn a(input: &str, map: &BTreeMap<usize, u32>, p2: bool) -> Option<Vec<Vec<u32>>> {
    let mut res = vec![];

    for s in Regex::new(r"[^0-9.\n]").ok()?.captures_iter(input) {
        let s = s.get(0)?;
        let mut set = HashSet::new();
        for pos in adjacent(s.start(), input) {
            if input.as_bytes()[pos].is_ascii_digit() {
                set.insert(map.range(..pos + 1).next_back()?.0);
            }
        }
        if !p2 || set.len() == 2 {
            res.push(set.iter().map(|pos| *map.get(pos).unwrap()).collect());
        }
    }
    Some(res)
}

fn adjacent(pos: usize, input: &str) -> Vec<usize> {
    let width = input.lines().next().unwrap().len() as isize + 1;
    let mut v = vec![];
    for i in -1..=1 {
        for j in -1..=1 {
            let x = (pos as isize % width) + j;
            let y = (pos as isize / width) + i;
            if (0..=input.len() as isize / width).contains(&y) && (0..=width).contains(&x) {
                v.push((pos as isize + i * width + j) as usize);
            }
        }
    }
    v
}
