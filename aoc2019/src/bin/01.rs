#[aors::main]
fn main(input: &str) -> (u32, i32) {
    let p1: u32 = input
        .lines()
        .map(|l| l.parse::<u32>().unwrap() / 3 - 2)
        .sum();
    let p2: i32 = input.lines().map(|l| fuel(l.parse().unwrap())).sum();
    (p1, p2)
}

fn fuel(w: i32) -> i32 {
    let f = w / 3 - 2;
    if f <= 0 {
        return 0;
    }
    f + fuel(f)
}
