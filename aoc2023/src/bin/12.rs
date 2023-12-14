use itertools::Itertools;

#[aors::main]
fn main(input: &str) -> (usize, usize) {
    let rows: Vec<(Vec<u8>, Vec<usize>)> = input
        .lines()
        .filter_map(|l| l.split_once(" "))
        .map(|(s, v)| {
            (
                s.as_bytes().to_vec(),
                v.split(",").filter_map(|n| n.parse().ok()).collect(),
            )
        })
        .collect();
    let rows2: Vec<(Vec<u8>, Vec<usize>)> = rows
        .iter()
        .map(|(r, n)| {
            (
                vec![r.clone(); 5]
                    .into_iter()
                    .intersperse(vec![b'?'])
                    .flatten()
                    .collect(),
                n.clone()
                    .into_iter()
                    .cycle()
                    .take(n.len() * 5)
                    .collect::<Vec<usize>>(),
            )
        })
        .collect();
    let p1 = rows.iter().filter_map(|(r, n)| rec(r, n, 0)).sum();
    // let p2 = rows2.iter().filter_map(|(r, n)| rec(r, n, 0)).sum();
    (p1, 0)
}

fn rec(r: &Vec<u8>, n: &Vec<usize>, pos: usize) -> Option<usize> {
    if pos >= r.len() {
        let s = std::str::from_utf8(&r).ok()?;
        let springs: Vec<usize> = s.split(".").map(|s| s.len()).filter(|l| l > &0).collect();
        if springs == *n {
            // println!("{} {:?}", s, n);
            Some(1)
        } else {
            None
        }
    } else if r[pos] == b'?' {
        let mut r1 = r.clone();
        r1[pos] = b'#';
        let mut r2 = r.clone();
        r2[pos] = b'.';
        Some(rec(&r1, n, pos + 1).unwrap_or(0) + rec(&r2, n, pos + 1).unwrap_or(0))
    } else {
        rec(r, n, pos + 1)
    }
}
