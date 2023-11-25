#[aors::main]
fn main(input: &str) -> (i64, i32) {
    let positions: Vec<u32> = input
        .split(',')
        .map(|n| n.parse().expect("Not a number"))
        .collect();
    (p1(&mut positions.clone()), p2(&positions))
}

fn p1(positions: &mut Vec<u32>) -> i64 {
    let median = median(positions);

    let mut fuel_use = 0;
    for pos in positions {
        fuel_use += (*pos as i64 - median as i64).abs();
    }

    fuel_use
}

fn p2(positions: &Vec<u32>) -> i32 {
    //height of the vec, i.e the largest number in it
    let height = positions.iter().max().unwrap();

    let mut min_fuel = i32::MAX;
    for middle in 0..*height {
        let mut fuel_use = 0;
        for pos in positions {
            let distance = (*pos as i32 - middle as i32).abs();
            fuel_use += distance * (distance + 1) / 2;
        }
        if fuel_use < min_fuel {
            min_fuel = fuel_use;
        }
    }

    min_fuel
}

fn median(vec: &mut Vec<u32>) -> u32 {
    vec.sort();
    let middle = vec.len() / 2;
    vec[middle]
}
