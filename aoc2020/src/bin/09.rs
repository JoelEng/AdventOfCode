use std::collections::VecDeque;

const WINDOW_SIZE: usize = 25; // 5 for example, 25 for real

#[aors::main]
fn main(input: &str) -> (u64, u64) {
    let nums: Vec<u64> = input.lines().map(|s| s.parse().unwrap()).collect();
    let mut window: VecDeque<u64> = VecDeque::new();
    for n in &nums[0..WINDOW_SIZE] {
        window.push_back(*n); // add preamble to window
    }

    let mut p1 = 0;
    for n in &nums[WINDOW_SIZE..] {
        if !check_valid(*n, &window) {
            p1 = *n;
            break;
        }
        window.pop_front();
        window.push_back(*n);
    }

    let p2;
    let mut i = 0;
    'outer: loop {
        window = VecDeque::new();
        window.push_back(nums[i]);
        let mut j = i + 1;
        loop {
            window.push_back(nums[j]);
            j += 1;
            let sum: u64 = window.iter().sum();
            if sum == p1 {
                let iter = window.iter();
                let smallest = iter.clone().max().unwrap();
                let largest = iter.min().unwrap();
                p2 = smallest + largest;
                break 'outer;
            } else if sum >= p1 {
                break;
            }
        }
        i += 1;
    }

    (p1, p2)
}

fn check_valid(num: u64, window: &VecDeque<u64>) -> bool {
    let window_size: u64 = WINDOW_SIZE.try_into().unwrap();
    for i in 0..window_size {
        let i_val = window[i.try_into().unwrap()];
        for j in (i + 1)..window_size {
            let j_val = window[j.try_into().unwrap()];
            if i_val + j_val == num {
                return true;
            }
        }
    }
    false
}
