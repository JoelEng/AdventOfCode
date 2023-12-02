use regex::Regex;

#[aors::main]
fn main(input: &str) -> (usize, u32) {
    let red = Regex::new("([0-9]+) red").ok().unwrap();
    let green = Regex::new("([0-9]+) green").ok().unwrap();
    let blue = Regex::new("([0-9]+) blue").ok().unwrap();

    let games = input
        .lines()
        .map(|g| (count(g, &red), count(g, &green), count(g, &blue)));
    let p1 = games
        .clone()
        .enumerate()
        .filter(|(_, (r, g, b))| *r <= 12 && *g <= 13 && *b <= 14)
        .map(|(i, _)| i + 1)
        .sum();
    (p1, games.map(|(r, g, b)| r * g * b).sum())
}

fn count(g: &str, re: &Regex) -> u32 {
    re.captures_iter(g)
        .filter_map(|c| c.get(1)?.as_str().parse().ok())
        .max()
        .unwrap_or(0)
}
