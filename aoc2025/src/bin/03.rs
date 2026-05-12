use itertools::Itertools;

mod helpers;

trait SumChars {
    fn sum_chars(&self) -> i128;
}

impl SumChars for Vec<Vec<char>> {
    fn sum_chars(&self) -> i128 {
        self.into_iter()
            .map(|v| v.iter().collect::<String>().parse::<i128>().unwrap())
            .sum()
    }
}

#[aors::main]
fn main(input: &str) -> (i128, i128) {
    (run(input, 2), run(input, 12))
}

fn run(input: &str, len: usize) -> i128 {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            let (mut s, mut res) = (l.to_string(), Vec::new());
            for i in (0..len).rev() {
                let a = cut_at_largest(s, i);
                s = a.1;
                res.push(a.0);
            }
            res
        })
        .collect::<Vec<Vec<char>>>()
        .sum_chars()
}

fn cut_at_largest(s: String, ignore_last: usize) -> (char, String) {
    let pos = s[..s.len() - ignore_last]
        .chars()
        .position_min_by_key(|c| -(c.to_digit(10).unwrap() as i32))
        .unwrap();
    let mut chars = s.chars();
    (chars.nth(pos).unwrap(), chars.collect())
}
