use memoize::memoize;

#[aors::main]
fn main(input: &str) -> (u32, u32) {
    let cards: Vec<(Vec<u32>, Vec<u32>)> = input
        .lines()
        .filter_map(|l| l.split(": ").last()?.split_once("|"))
        .map(|(w, y)| (parse(w), parse(y)))
        .collect();
    let p1 = cards
        .iter()
        .map(|(w, y)| 2_u32.pow(wins(w, y) as u32 - 1))
        .sum();
    let mut p2 = 0;
    for (i, _) in cards.iter().enumerate() {
        p2 += new_cards(i, cards.clone());
    }
    (p1, p2)
}

#[memoize]
fn new_cards(i: usize, cards: Vec<(Vec<u32>, Vec<u32>)>) -> u32 {
    let mut count = 0;
    if i < cards.len() {
        count += 1;
        let (w, y) = cards.get(i).unwrap();
        for i in i + 1..=i + wins(w, y) {
            count += new_cards(i, cards.clone());
        }
    }
    count
}

fn wins(winning: &Vec<u32>, yours: &Vec<u32>) -> usize {
    let a: Vec<&u32> = winning.iter().filter(|n| yours.contains(n)).collect();
    a.len()
}

fn parse(s: &str) -> Vec<u32> {
    s.split_whitespace()
        .filter_map(|n| n.parse().ok())
        .collect()
}
