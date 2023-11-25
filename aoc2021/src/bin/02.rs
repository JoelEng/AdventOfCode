#[aors::main]
fn main(input: &str) -> (u32, i32) {
    let input: Vec<(char, u32)> = input
        .lines()
        .map(|word| {
            (
                word.chars().next().unwrap(),
                word.chars().last().unwrap().to_digit(10).unwrap(),
            )
        })
        .collect();
    (p1(&input), p2(&input))
}

fn p1(input: &Vec<(char, u32)>) -> u32 {
    let mut forward = 0;
    let mut depth = 0;

    for (direction, steps) in input {
        match direction {
            'f' => forward += steps,
            'u' => depth -= steps,
            'd' => depth += steps,
            _ => (),
        }
    }

    forward * depth
}

fn p2(input: &Vec<(char, u32)>) -> i32 {
    let mut forward = 0;
    let mut depth = 0;
    let mut aim: i32 = 0;

    for (direction, steps) in input {
        let steps = *steps as i32;

        match direction {
            'f' => {
                forward += steps;
                depth += aim * steps;
            }
            'u' => {
                aim -= steps;
            }
            'd' => {
                aim += steps;
            }
            _ => (),
        }
    }

    forward as i32 * depth
}
