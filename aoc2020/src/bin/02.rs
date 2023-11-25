use regex::Regex;

#[aors::main]
fn main(input: &str) -> (usize, usize) {
    let v = get_input(input);
    let res = v.into_iter().map(|v| check_valid(&v));
    let p1 = res.clone().filter(|(x, _)| *x).collect::<Vec<_>>().len();
    let p2 = res.filter(|(_, x)| *x).collect::<Vec<_>>().len();
    (p1, p2)
}

fn check_valid(v: &Vec<String>) -> (bool, bool) {
    let lower = v[0].parse::<i32>().unwrap();
    let upper = v[1].parse::<i32>().unwrap();
    let letter = v[2].as_str().chars().nth(0).unwrap();
    let word = v[3].as_str();

    (
        part_one(lower, upper, letter, word),
        part_two(lower, upper, letter, word),
    )
}

fn part_one(lower: i32, upper: i32, letter: char, word: &str) -> bool {
    let re = Regex::new(&letter.to_string()).unwrap();
    let count = re
        .find_iter(word)
        .map(|c| c.as_str())
        .collect::<Vec<&str>>()
        .len();

    count <= upper.try_into().unwrap() && count >= lower.try_into().unwrap()
}

fn part_two(lower: i32, upper: i32, letter: char, word: &str) -> bool {
    let first: char = word.chars().nth((lower - 1).try_into().unwrap()).unwrap();
    let second: char = word.chars().nth((upper - 1).try_into().unwrap()).unwrap();

    (first == letter) ^ (second == letter)
}

fn get_input(input: &str) -> Vec<Vec<String>> {
    let re = Regex::new(r"\w+").unwrap();
    input
        .lines()
        .map(|s| {
            re.find_iter(s)
                .filter_map(|digits| digits.as_str().parse().ok())
                .collect()
        })
        .collect()
}
