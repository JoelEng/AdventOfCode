#[aors::main]
fn main(input: &str) -> (i32, i32) {
    let lines = input.lines().map(|l| {
        l.split_whitespace()
            .filter_map(|n| n.parse().ok())
            .collect::<Vec<_>>()
    });
    let (l, r): (Vec<_>, Vec<_>) = lines.map(|l| run(l)).unzip();
    (r.iter().sum(), l.iter().sum())
}

fn run(v: Vec<i32>) -> (i32, i32) {
    if v.iter().all(|n| *n == 0) {
        return (0, 0);
    }
    let (l, r) = run(v.windows(2).map(|w| w[1] - w[0]).collect());
    (v[0] - l, v[v.len() - 1] + r)
}
