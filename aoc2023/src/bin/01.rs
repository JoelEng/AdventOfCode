const WORDS: &[&str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

#[aors::main]
fn main(input: &str) -> (u32, u32) {
    (sum(input, false), sum(input, true))
}

fn sum(input: &str, part2: bool) -> u32 {
    input.lines().filter_map(|l| line_val(l, part2)).sum()
}

fn line_val(line: &str, part2: bool) -> Option<u32> {
    let v: Vec<u32> = line
        .chars()
        .enumerate()
        .filter_map(|(pos, c)| val(line, pos, c, part2))
        .collect();
    Some(v.first()? * 10 + v.last()?)
}

fn val(line: &str, pos: usize, c: char, part2: bool) -> Option<u32> {
    match c {
        d if d.is_digit(10) => d.to_digit(10),
        _ if part2 => WORDS
            .iter()
            .enumerate()
            .find_map(|(i, w)| line[pos..].starts_with(w).then_some(i as u32 + 1)),
        _ => None,
    }
}
