mod helpers;
use helpers::get_digit;
use itertools::Itertools;

#[aors::main]
fn main(input: &str) -> (usize, usize) {
    let (start, end) = input
        .split("-")
        .flat_map(|s| s.parse::<u32>())
        .next_tuple()
        .unwrap();
    let p1 = (start..end).map(|i| count(i, false)).filter(|i| *i).count();
    let p2 = (start..end).map(|i| count(i, true)).filter(|i| *i).count();
    (p1, p2)
}

fn count(i: u32, p2: bool) -> bool {
    let mut prev = 10;
    let mut pair_len = 1;
    let mut has_pair = false;
    for d in 0..6 {
        let digit = get_digit(i, d, 1);
        if digit > prev {
            return false;
        }
        if digit == prev {
            pair_len += 1;
        } else {
            if pair_len == 2 || (pair_len > 2 && !p2) {
                has_pair = true;
            }
            pair_len = 1;
        }

        prev = digit;
    }
    if pair_len == 2 || (pair_len > 2 && !p2) {
        has_pair = true;
    }
    has_pair
}
