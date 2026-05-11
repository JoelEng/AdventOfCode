mod helpers;

#[aors::main]
fn main(input: &str) -> (i32, i32) {
    let v: Vec<i32> = input
        .replace("L", "-")
        .replace("R", "")
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();
    let (mut p1, mut p2) = (0, 0);
    v.iter().fold(50, |acc, x| rot(acc, *x, &mut p1));
    v.iter().fold(50, |acc, x| {
        let x1 = x / x.abs();
        (0..x.abs()).fold(acc, |acc, _| rot(acc, x1, &mut p2))
    });
    (p1, p2)
}

fn rot(acc: i32, x: i32, i: &mut i32) -> i32 {
    let res = (acc + x) % 100;
    if res == 0 {
        *i += 1
    };
    res
}
