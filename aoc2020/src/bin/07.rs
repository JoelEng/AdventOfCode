use hashbrown::HashMap;

const BAG: &str = "shiny gold";
type ContentMap = HashMap<String, HashMap<String, u32>>;

#[aors::main]
fn main(input: &str) -> (usize, u32) {
    let map = get_map(input);

    let p1 = contents(BAG, &reverse_map(&map), 1);

    let p2 = contents(BAG, &map, 1);

    (p1.len(), p2.values().sum::<u32>())
}

/**
 * Finds all connections from a node in a directed acyclic graph.
 */
fn contents(id: &str, map: &ContentMap, mul: u32) -> HashMap<String, u32> {
    let mut a = HashMap::new();
    let bags = match map.get(id) {
        Some(x) => x,
        None => return a,
    };
    for (bag, i) in bags {
        *a.entry(bag.to_string()).or_default() += *i * mul;
        for (child, i_child) in contents(bag, map, *i * mul) {
            *a.entry(child.to_string()).or_default() += i_child;
        }
    }
    a
}

fn reverse_map(map: &ContentMap) -> ContentMap {
    let mut new_map: ContentMap = HashMap::new();
    for (k, v) in map {
        for (new_k, i) in v {
            let entry = new_map.entry(new_k.clone()).or_default();
            entry.insert(k.clone(), *i);
        }
    }
    new_map
}

/**
 * returns a map of every bag along with its contents,
 * as well as how many times each item is in it.
 *
 * i.e:
 * "muted yellow": ["shiny gold": 2]
 * means that a muted yellow bag directly contains 2 shiny gold bags
 */
fn get_map(input: &str) -> ContentMap {
    let mut contains: ContentMap = HashMap::new();

    for row in input.split("\n") {
        let mut split = row.split("bags contain");
        let outer_bag = split.nth(0).unwrap().trim();
        let contents = split.nth(0).unwrap().replace(".", "");
        let inner_bags = contains.entry(outer_bag.to_string()).or_default();

        for b in contents.split(",") {
            let b = b.replace("bags", "").replace("bag", "");
            let b = b.trim();
            if b == "no other" {
                continue;
            }

            let (count, inner_bag) = b.split_once(" ").unwrap();
            let count: u32 = count.parse().unwrap();

            inner_bags.entry(inner_bag.to_string()).or_insert(count);
        }
    }
    contains
}
