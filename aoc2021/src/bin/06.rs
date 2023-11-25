const ARRAY_SIZE: usize = 9;

#[aors::main]
fn main(input: &str) -> (u64, u64) {
    let mut fish: [u64; ARRAY_SIZE] = [0; ARRAY_SIZE];
    for i in input.split(',') {
        fish[i.parse::<usize>().expect("Not a valid number")] += 1;
    }

    (solve(80, &mut fish.clone()), solve(256, &mut fish))
}

fn solve(days: usize, fish: &mut [u64; ARRAY_SIZE]) -> u64 {
    for _ in 0..days {
        decrease_timers(fish);
    }
    fish.iter().sum::<u64>()
}

fn decrease_timers(fish: &mut [u64; ARRAY_SIZE]) {
    let fish0 = fish[0];
    for i in 0..(ARRAY_SIZE - 1) {
        fish[i] = fish[i + 1];
    }
    fish[ARRAY_SIZE - 1] = fish0;
    fish[ARRAY_SIZE - 3] += fish0;
}
