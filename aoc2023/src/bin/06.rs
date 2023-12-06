use itertools::Itertools;

#[aors::main]
fn main(input: &str) -> (u128, u128) {
    let (times, records): (Vec<u128>, Vec<u128>) = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .filter_map(|n| n.parse().ok())
                .collect()
        })
        .next_tuple()
        .unwrap();
    println!("{:?}", records);
    let p1 = times.into_iter().zip(records).collect();
    (run(p1), run(vec![(49787980, 298118510661181)]))
}

fn run(races: Vec<(u128, u128)>) -> u128 {
    let mut wins = vec![];
    for (time, record) in races {
        for hold in 1..time {
            if (time - hold) * hold > record {
                wins.push(time - (hold * 2) + 1);
                break;
            }
        }
    }
    wins.iter().product()
}
