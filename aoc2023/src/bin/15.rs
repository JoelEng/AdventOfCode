#[aors::main]
fn main(input: &str) -> (usize, usize) {
    let mut boxes: Vec<Vec<(String, usize)>> = vec![vec![]; 256];
    for step in input.split(",") {
        let label: String = step.chars().filter(|c| c.is_alphabetic()).collect();
        let b = &mut boxes[hash(&label)];
        if step.ends_with("-") {
            *b = b.clone().into_iter().filter(|(s, _)| *s != label).collect();
        } else {
            let focal: usize = step
                .chars()
                .filter(|c| c.is_digit(10))
                .collect::<String>()
                .parse()
                .unwrap();
            if let Some(pos) = b.iter().position(|(s, _)| *s == label) {
                b[pos].1 = focal;
            } else {
                b.push((label, focal));
            }
        }
    }
    let p2 = boxes
        .iter()
        .enumerate()
        .map(|(i, b)| {
            (i + 1)
                * b.iter()
                    .enumerate()
                    .map(|(i, (_, f))| (i + 1) * f)
                    .sum::<usize>()
        })
        .sum();
    (input.split(",").map(|s| hash(s)).sum(), p2)
}

fn hash(s: &str) -> usize {
    s.as_bytes().iter().fold(0, |mut acc, c| {
        acc += *c as usize;
        acc *= 17;
        acc %= 256;
        acc
    })
}
