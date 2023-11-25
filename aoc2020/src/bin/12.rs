#[aors::main]
fn main(input: &str) -> (i32, i32) {
    let instrs: Vec<(char, i32)> = input
        .lines()
        .map(|s| {
            let s = s.split_at(1);
            (s.0.chars().next().unwrap(), s.1.parse().unwrap())
        })
        .collect();

    (p1(&instrs), p2(&instrs))
}

fn p1(instrs: &Vec<(char, i32)>) -> i32 {
    let mut dir: i32 = 0;
    let mut pos: (i32, i32) = (0, 0);
    for (action, value) in instrs {
        match action {
            'E' => pos.0 += value,
            'N' => pos.1 += value,
            'W' => pos.0 -= value,
            'S' => pos.1 -= value,
            'L' => dir += value,
            'R' => dir -= value,
            'F' => match dir.rem_euclid(360) {
                0 => pos.0 += value,
                90 => pos.1 += value,
                180 => pos.0 -= value,
                270 => pos.1 -= value,
                _ => unreachable!(),
            },
            _ => (),
        }
    }
    pos.0.abs() + pos.1.abs()
}

fn p2(instrs: &Vec<(char, i32)>) -> i32 {
    let mut pos = (0, 0);
    let mut way_pos = (10, 1); // relative to the ship
    for (action, value) in instrs {
        match action {
            'E' => way_pos.0 += value,
            'N' => way_pos.1 += value,
            'W' => way_pos.0 -= value,
            'S' => way_pos.1 -= value,
            'L' => {
                for _ in 0..(value / 90) {
                    let temp = way_pos.0;
                    way_pos.0 = -way_pos.1;
                    way_pos.1 = temp;
                }
            }
            'R' => {
                for _ in 0..(value / 90) {
                    let temp = way_pos.0;
                    way_pos.0 = way_pos.1;
                    way_pos.1 = -temp;
                }
            }
            'F' => {
                pos.0 += way_pos.0 * value;
                pos.1 += way_pos.1 * value;
            }
            _ => unreachable!(),
        }
    }
    pos.0.abs() + pos.1.abs()
}
