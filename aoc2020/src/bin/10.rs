use itertools::Itertools;

#[aors::main]
fn main(input: &str) -> (u32, usize) {
    let mut joltage: Vec<u32> = input.lines().map(|s| s.parse::<u32>().unwrap()).collect();
    joltage.push(0);
    joltage.sort();
    joltage.push(joltage[joltage.len() - 1] + 3);

    let mut diff_1 = 0;
    let mut diff_3 = 0;
    for (a, b) in joltage.iter().tuple_windows() {
        let diff = b - a;
        if diff == 1 {
            diff_1 += 1;
        } else {
            diff_3 += 1; // There are no consecutive adapters with a diff of 2
        }
    }

    let mut prod: usize = 1;
    let mut a = 0;
    while a < joltage.len() - 1 {
        let mut i = a;
        while joltage[i + 1] - joltage[i] == 1 {
            i += 1;
        }
        let size = i - a + 1;
        if size > 2 {
            // clusters of diff_1 with size > 2 is the only way to acheive more variations
            let mut p = 1;
            for n in 2..size {
                p += n - 1;
            }
            prod *= p; // this is wrong
        }
        a = i + 1;
    }

    (diff_1 * diff_3, prod)
}
