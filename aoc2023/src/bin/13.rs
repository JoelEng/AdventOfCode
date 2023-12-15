#[aors::main]
fn main(input: &str) -> (usize, usize) {
    let mut patterns: Vec<Vec<Vec<bool>>> = input
        .split("\n\n")
        .map(|p| {
            p.lines()
                .map(|l| l.chars().map(|c| c == '#').collect())
                .collect()
        })
        .collect();
    let (mut p1, mut p2) = (0, 0);
    for p in &mut patterns {
        let a = find(p, 0);
        let s = smudge(p, a);
        p1 += 100 * a;
        p2 += 100 * s;
        let mut tp = (0..p[0].len())
            .map(|i| p.iter().map(|v| v[i]).collect())
            .collect();
        let a = find(&tp, 0);
        p1 += a;
        if s == 0 {
            p2 += smudge(&mut tp, a);
        }
    }
    (p1, p2)
}

fn smudge(p: &mut Vec<Vec<bool>>, not: usize) -> usize {
    for y in 0..p.len() {
        for x in 0..p[0].len() {
            p[y][x] = !p[y][x];
            let res = find(p, not);
            p[y][x] = !p[y][x];
            if res != 0 {
                return res;
            }
        }
    }
    0
}

fn find(p: &Vec<Vec<bool>>, not: usize) -> usize {
    'outer: for i in 1..p.len() {
        if i == not {
            continue;
        }
        let size = i.min(p.len() - i);
        for offset in 0..size {
            if p[i - offset - 1] != p[i + offset] {
                continue 'outer;
            }
        }
        return i;
    }
    0
}
