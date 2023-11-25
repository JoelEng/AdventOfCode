use itertools::Itertools;
use std::collections::HashMap;

#[aors::main]
fn main(input: &str) -> (u64, u64) {
    (solve(10, input), solve(40, input))
}

fn get_input(
    input: &str,
) -> (
    HashMap<String, u64>,
    HashMap<char, u64>,
    HashMap<String, (String, String)>,
) {
    let mut lines = input.lines();

    let mut rules = HashMap::new();
    let init_vals = lines.next().unwrap().to_string();
    let mut polymer = HashMap::new();
    let mut counter = HashMap::new();

    lines.next(); //Remove blank line

    //Create rules
    for line in lines {
        let pair = &line[0..2];
        let insertion = line.chars().last().unwrap();
        let mut chars = pair.chars();
        rules.insert(
            String::from(pair),
            (
                format!("{}{}", chars.next().unwrap(), insertion),
                format!("{}{}", insertion, chars.next().unwrap()),
            ),
        );
    }
    //Create polymer
    for tuple in init_vals.chars().tuple_windows::<(char, char)>() {
        let tuple = polymer
            .entry(format!("{}{}", tuple.0, tuple.1))
            .or_insert(0);
        *tuple += 1;
    }
    //Create counter
    for c in init_vals.chars() {
        let count = counter.entry(c).or_insert(0);
        *count += 1;
    }

    (polymer, counter, rules)
}

fn solve(iterations: u32, input: &str) -> u64 {
    let (mut polymer, mut counter, rules) = get_input(input);

    //Insertions
    for _ in 0..iterations {
        let mut old_polymer = polymer.clone();
        for (key, (value1, value2)) in &rules {
            let val = old_polymer.entry(key.clone()).or_insert(0);
            let val = *val;

            let parent = polymer.entry(key.clone()).or_insert(0);
            *parent -= val;

            let child1 = polymer.entry(value1.clone()).or_insert(0);
            *child1 += val;

            let child2 = polymer.entry(value2.clone()).or_insert(0);
            *child2 += val;

            let count = counter.entry(value1.chars().last().unwrap()).or_insert(0);
            *count += val;
        }
    }

    let most_common = counter.iter().max_by_key(|e| e.1).unwrap();
    let least_common = counter.iter().min_by_key(|e| e.1).unwrap();
    most_common.1 - least_common.1
}
