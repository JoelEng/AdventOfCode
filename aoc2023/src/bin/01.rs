const WORDS: &[&str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

#[aors::main]
fn main(input: &str) -> (u32, u32) {
    (sum(input, false), sum(input, true))
}

fn sum(input: &str, part2: bool) -> u32 {
    input
        .lines()
        .map(|l| {
            l.chars()
                .enumerate()
                .filter_map(|(pos, c)| match c {
                    d if d.is_digit(10) => d.to_digit(10),
                    _ if part2 => WORDS
                        .iter()
                        .enumerate()
                        .find_map(|(i, w)| l[pos..].starts_with(w).then_some(i as u32 + 1)),
                    _ => None,
                })
                .collect::<Vec<u32>>()
        })
        .filter_map(|l| Some(l.first()? * 10 + l.last()?))
        .sum()
}
