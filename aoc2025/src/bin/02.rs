use itertools::Itertools;

mod helpers;

#[aors::main]
fn main(input: &str) -> (i128, i128) {
    let (mut p1, mut p2): (i128, i128) = (0, 0);
    for s in input.split(",") {
        let (a, b) = s.split_once("-").unwrap();
        let (a, b): (i128, i128) = (a.parse().unwrap(), b.parse().unwrap());
        for i in a..=b {
            let is = i.to_string();
            let len = is.len();
            let (i1, i2) = is.split_at(len / 2);
            if i1 == i2 {
                p1 += i;
            }
            for div in 1..=len / 2 {
                if len % div == 0
                    && is
                        .chars()
                        .chunks(div)
                        .into_iter()
                        .map(|c| c.collect::<String>())
                        .all_equal()
                {
                    p2 += i;
                    break;
                }
            }
        }
    }
    (p1, p2)
}
