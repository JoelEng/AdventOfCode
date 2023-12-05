use itertools::Itertools;

#[aors::main]
fn main(input: &str) -> (u32, i32) {
    let (seeds, maps) = parse(input);

    let mut p1 = vec![];
    for seed in seeds {
        let mut val = seed;
        for m in &maps {
            for (dest, src, len) in m {
                if (*src..src + len).contains(&val) {
                    val = val + (dest - src);
                    break;
                }
            }
        }
        p1.push(val);
    }
    (*p1.iter().min().unwrap(), 0)
}

fn parse(input: &str) -> (Vec<u32>, Vec<Vec<(u32, u32, u32)>>) {
    let mut input = input.split("\n\n");
    let seeds: Vec<u32> = input
        .next()
        .unwrap()
        .split_whitespace()
        .filter_map(|n| n.parse().ok())
        .collect();
    let maps: Vec<Vec<(u32, u32, u32)>> = input
        .map(|m| {
            m.split_whitespace()
                .filter_map(|n| n.parse().ok())
                .tuples()
                .collect()
        })
        .collect();
    (seeds, maps)
}
