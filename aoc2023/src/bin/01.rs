const WORDS: &[&str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

#[aors::main]
fn main(input: &str) -> (u32, u32) {
    (sum(input, false), sum(input, true))
}

fn sum(input: &str, p2: bool) -> u32 {
    input.lines().filter_map(|l| line_val(l, p2)).sum()
}

fn line_val(line: &str, p2: bool) -> Option<u32> {
    let v: Vec<u32> = line
        .chars()
        .enumerate()
        .filter_map(|(i, c)| val(line, i, c, p2))
        .collect();
    Some(v.first()? * 10 + v.last()?)
}

fn val(line: &str, i: usize, c: char, p2: bool) -> Option<u32> {
    c.to_digit(10)
        .or_else(|| p2.then_some(WORDS.iter().position(|w| line[i..].starts_with(w))? as u32 + 1))
}
