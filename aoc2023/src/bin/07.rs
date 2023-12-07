use itertools::Itertools;
use std::cmp::Ordering;

#[aors::main]
fn main(input: &str) -> (u32, u32) {
    let s1 = &[
        '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
    ];
    let s2 = &[
        'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
    ];
    let mut hands: Vec<(&str, u32)> = input
        .lines()
        .filter_map(|l| l.split_whitespace().next_tuple())
        .map(|(hand, bid)| (hand, bid.parse().unwrap()))
        .collect();
    let p1 = winnings(&mut hands, s1, false);
    let p2 = winnings(&mut hands, s2, true);
    (p1, p2)
}

fn winnings(hands: &mut Vec<(&str, u32)>, strength: &[char], p2: bool) -> u32 {
    hands.sort_by(|(a, _), (b, _)| order(a, b, strength, p2));
    hands
        .iter()
        .enumerate()
        .map(|(i, (_, b))| (i as u32 + 1) * b)
        .sum()
}

fn order(a: &str, b: &str, strength: &[char], p2: bool) -> Ordering {
    if hand_type(a, p2).cmp(&hand_type(b, p2)) != Ordering::Equal {
        return hand_type(a, p2).cmp(&hand_type(b, p2));
    }
    for (a, b) in a.chars().zip(b.chars()) {
        if a != b {
            let a = strength.iter().position(|c| *c == a).unwrap();
            let b = strength.iter().position(|c| *c == b).unwrap();
            return a.cmp(&b);
        }
    }
    unreachable!()
}

fn hand_type(h: &str, p2: bool) -> i32 {
    let mut v = vec![];
    let mut chars: Vec<char> = h.chars().collect();
    let mut jokers = 0;
    if p2 {
        jokers = chars.iter().filter(|c| **c == 'J').count();
        chars = chars.into_iter().filter(|c| *c != 'J').collect();
    }
    while let Some(card) = chars.pop() {
        let count = chars.iter().filter(|c| **c == card).count();
        if count > 0 {
            v.push(count + 1);
            chars = chars.into_iter().filter(|c| *c != card).collect();
        }
    }
    if !v.is_empty() {
        let i = v.iter().enumerate().max_by(|a, b| a.1.cmp(b.1)).unwrap().0;
        v[i] += jokers;
    } else {
        v.push(jokers + 1);
    }
    match v.as_slice() {
        [5] | [6] => 6,
        [4] => 5,
        [2, 3] | [3, 2] => 4,
        [3] => 3,
        [2, 2] => 2,
        [2] => 1,
        _ => 0,
    }
}
