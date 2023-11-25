use std::str::Split;

#[aors::main]
fn main(input: &str) -> (i32, i32) {
    let mut lines = input.lines();
    let earliest: i32 = lines.next().unwrap().parse().unwrap();
    let busses = lines.next().unwrap().split(",");

    (p1(earliest, busses.clone()), p2(busses))
}

fn p1(earliest: i32, busses: Split<&str>) -> i32 {
    let busses: Vec<i32> = busses.clone().filter_map(|b| b.parse().ok()).collect();
    let mut first_bus = (i32::MAX, 0); // (minutes from earliest timestamp, bus id)

    for bus in busses {
        let time = (earliest % bus - bus).abs();
        if time < first_bus.0 {
            first_bus = (time, bus);
        }
    }
    first_bus.0 * first_bus.1
}

fn p2(busses: Split<&str>) -> i32 {
    let mut offset = -1;
    let busses: Vec<(i32, i32)> = busses
        .filter_map(|b| {
            offset += 1;
            match b.parse::<i32>() {
                Ok(id) => Some((id, offset)),
                Err(_) => None,
            }
        })
        .collect();

    println!("{:?}", busses);
    0
}
