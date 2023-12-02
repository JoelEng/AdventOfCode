use regex::Regex;

#[aors::main]
fn main(input: &str) -> (usize, u32) {
    let games: Vec<Vec<&str>> = input
        .lines()
        .filter_map(|l| l.split(":").last())
        .map(|g| g.split(";").collect())
        .collect();
    let p1 = games
        .iter()
        .enumerate()
        .filter(|(_, g)| game(g))
        .map(|(i, _)| i + 1)
        .sum();
    let p2 = games.iter().map(|g| game2(g)).sum();
    (p1, p2)
}

fn game(g: &Vec<&str>) -> bool {
    let red = Regex::new("([0-9]+) red").ok().unwrap();
    let green = Regex::new("([0-9]+) green").ok().unwrap();
    let blue = Regex::new("([0-9]+) blue").ok().unwrap();

    possible(g, &red, 12) && (possible(g, &green, 13)) && (possible(g, &blue, 14))
}

fn possible(g: &Vec<&str>, r: &Regex, max: u32) -> bool {
    let n: Option<u32> = g
        .iter()
        .filter_map(|s| r.captures(s)?.get(1)?.as_str().parse().ok())
        .max();
    if let Some(n) = n {
        return n <= max;
    }
    false
}

fn game2(g: &Vec<&str>) -> u32 {
    let red = Regex::new("([0-9]+) red").ok().unwrap();
    let green = Regex::new("([0-9]+) green").ok().unwrap();
    let blue = Regex::new("([0-9]+) blue").ok().unwrap();

    possible2(g, &red) * (possible2(g, &green)) * (possible2(g, &blue))
}

fn possible2(g: &Vec<&str>, r: &Regex) -> u32 {
    g.iter()
        .filter_map(|s| r.captures(s)?.get(1)?.as_str().parse().ok())
        .max()
        .unwrap_or(0)
}
