use hashbrown::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

type Group = Vec<Vec<char>>;

#[aors::main]
fn main(input: &str) -> (usize, usize) {
    let groups = get_groups(input);

    let mut p1 = 0;
    for group in &groups {
        let ans: Vec<&char> = group.iter().flatten().collect();
        let set: HashSet<&char> = HashSet::from_iter(ans);
        p1 += set.len();
    }

    let mut p2 = 0;
    for group in &groups {
        let group_size = group.len();
        let ans: Vec<&char> = group.iter().flatten().collect();

        let mut counts: HashMap<char, usize> = HashMap::new();
        for x in ans {
            *counts.entry(*x).or_default() += 1;
        }

        for (_, n) in counts {
            if n == group_size {
                p2 += 1;
            }
        }
    }
    (p1, p2)
}

fn get_groups(input: &str) -> Vec<Group> {
    input
        .split("\n\n")
        .map(|group| {
            group
                .split_whitespace()
                .map(|p| p.chars().collect())
                .collect()
        })
        .collect()
}
