mod helpers;

const WORDS: &[&str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

#[aors::main]
fn main(input: &str) -> (u32, u32) {
    let part2: String = input
        .lines()
        .map(|l| {
            let mut digits = "\n".to_string();
            for pos in 0..l.len() {
                if let Some(d) = l.chars().nth(pos).unwrap().to_digit(10) {
                    digits.push_str(&d.to_string());
                }
                for (i, word) in WORDS.iter().enumerate() {
                    if l[pos..].starts_with(word) {
                        digits.push_str(&(i + 1).to_string())
                    }
                }
            }
            digits
        })
        .collect();
    (sum(input), sum(part2.as_str()))
}

fn sum(input: &str) -> u32 {
    let digits: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.chars().filter_map(|c| c.to_digit(10)).collect())
        .collect();

    digits
        .iter()
        .filter_map(|l| Some(l.first()? * 10 + l.last()?))
        .sum()
}
