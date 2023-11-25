const SUM_TO: i32 = 2020;

// main(01) runs with actual input, main(01, 0) runs with example input
#[aors::main]
fn main(input: &str) -> (i32, i32) {
    let v = input
        .lines()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    find_sums(v)
}

fn find_sums(v: Vec<i32>) -> (i32, i32) {
    let (mut p1, mut p2) = (0, 0);
    for i in &v {
        for j in &v {
            if i + j == SUM_TO {
                p1 = i * j;
            }
            for k in &v {
                if i + j + k == SUM_TO {
                    p2 = i * j * k;
                }
            }
        }
    }
    return (p1, p2);
}
