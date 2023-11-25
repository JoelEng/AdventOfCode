#[aors::main]
fn main(input: &str) -> (u32, usize) {
    let input: Vec<Vec<String>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|word| word.to_string())
                .collect::<Vec<String>>()
        })
        .collect();

    let input: Vec<(Vec<String>, Vec<String>)> = input
        .iter()
        .map(|line| {
            let mut split = line.split(|word| word == "|");
            (
                Vec::from(split.next().unwrap()),
                Vec::from(split.next().unwrap()),
            )
        })
        .collect();
    (p1(&input), p2(input.clone()))
}

fn p1(input: &Vec<(Vec<String>, Vec<String>)>) -> u32 {
    let output: Vec<Vec<String>> = input.iter().map(|(_, o)| o.clone()).collect();
    let mut nbr_of_words = [0u32; 10];

    for word in output.iter().flatten() {
        match word.len() {
            2 => nbr_of_words[1] += 1,
            4 => nbr_of_words[4] += 1,
            3 => nbr_of_words[7] += 1,
            7 => nbr_of_words[8] += 1,
            _ => (),
        }
    }

    nbr_of_words.iter().sum::<u32>()
}

fn p2(lines: Vec<(Vec<String>, Vec<String>)>) -> usize {
    let mut sum = 0;

    for line in lines {
        let mut value = 0;
        let nbrs = solve_line(line.0);
        let mut output = line.1.iter();
        let (a, b, c, d) = (
            output.next().unwrap(),
            output.next().unwrap(),
            output.next().unwrap(),
            output.next().unwrap(),
        );
        value += nbrs
            .iter()
            .position(|n| difference(n, a).len() == 0)
            .unwrap()
            * 1000;
        value += nbrs
            .iter()
            .position(|n| difference(n, b).len() == 0)
            .unwrap()
            * 100;
        value += nbrs
            .iter()
            .position(|n| difference(n, c).len() == 0)
            .unwrap()
            * 10;
        value += nbrs
            .iter()
            .position(|n| difference(n, d).len() == 0)
            .unwrap()
            * 1;

        sum += value;
    }

    sum
}

fn solve_line(input: Vec<String>) -> [String; 10] {
    let mut nbrs: [&str; 10] = Default::default();

    for word in input.iter() {
        match word.len() {
            2 => nbrs[1] = word,
            4 => nbrs[4] = word,
            3 => nbrs[7] = word,
            7 => nbrs[8] = word,
            _ => (),
        }
    }

    for word in input.iter() {
        if word.len() == 6 {
            match difference(word, nbrs[4]).len() {
                2 => nbrs[9] = word,
                _ => {
                    if difference(word, nbrs[1]).len() == 4 {
                        nbrs[0] = word;
                    } else {
                        nbrs[6] = word
                    }
                }
            }
        }
    }

    let e = difference(nbrs[8], nbrs[9]);

    for word in input.iter() {
        if word.len() == 5 {
            match difference(word, nbrs[6]).len() {
                1 => nbrs[5] = word,
                _ => {
                    if word.contains(|n| n == e.chars().next().unwrap()) {
                        nbrs[2] = word
                    } else {
                        nbrs[3] = word
                    }
                }
            }
        }
    }

    nbrs.map(|s| s.to_string())
}

fn difference(first: &str, second: &str) -> String {
    let diff1: String = first.chars().filter(|c| !second.contains(*c)).collect();
    let diff2: String = second.chars().filter(|c| !first.contains(*c)).collect();
    diff1 + &diff2
}
