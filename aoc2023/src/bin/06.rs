use itertools::Itertools;

#[aors::main]
fn main(input: &str) -> (u32, u32) {
    let mut a: Vec<f64> = input
        .split_whitespace()
        .filter_map(|w| w.parse().ok())
        .collect();
    let p1 = a.split_off(a.len() / 2).into_iter().zip(a);
    let (t, r) = input.lines().map(|l| p2(l)).next_tuple().unwrap();
    (p1.map(|(r, t)| calc(t, r)).product(), calc(t, r))
}

fn calc(t: f64, r: f64) -> u32 {
    t as u32 - 1 - 2 * ((t / 2.0) - ((t / 2.0).powf(2.0) - r).sqrt()) as u32
}

fn p2(l: &str) -> f64 {
    l.chars()
        .filter_map(|c| c.to_digit(10))
        .fold(0.0, |acc, n| acc * 10.0 + n as f64)
}
