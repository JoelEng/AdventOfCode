use hashbrown::HashMap;

#[aors::main]
fn main(input: &str) -> (u32, u32) {
    let cards: Vec<(Vec<u32>, Vec<u32>)> = input
        .lines()
        .filter_map(|l| l.split(": ").last()?.split_once("|"))
        .map(|(w, y)| (parse(w), parse(y)))
        .collect();
    let p1 = cards.iter().map(|c| 2_u32.pow(wins(c) as u32 - 1)).sum();
    let mut memo = HashMap::new();
    let p2 = (0..cards.len()).map(|i| count(i, &cards, &mut memo)).sum();
    (p1, p2)
}

fn count(i: usize, cards: &Vec<(Vec<u32>, Vec<u32>)>, memo: &mut HashMap<usize, u32>) -> u32 {
    if let Some(c) = memo.get(&i) {
        return *c;
    }
    let n: u32 = (i + 1..=i + wins(&cards[i]))
        .map(|i| count(i, cards, memo))
        .sum();
    memo.insert(i, n + 1);
    n + 1
}

fn wins(cards: &(Vec<u32>, Vec<u32>)) -> usize {
    let a: Vec<&u32> = cards.0.iter().filter(|n| cards.1.contains(n)).collect();
    a.len()
}

fn parse(s: &str) -> Vec<u32> {
    s.split_whitespace()
        .filter_map(|n| n.parse().ok())
        .collect()
}
