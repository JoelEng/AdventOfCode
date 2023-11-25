#[aors::main]
fn main(input: &str) -> (i32, i32) {
    let input = input.split_whitespace();
    let nums: Vec<i32> = input
        .map(|word| {
            word.parse::<i32>()
                .expect("Wrong input file. Something was not a number")
        })
        .collect();
    (p1(&nums), p2(&nums))
}

fn p1(nums: &Vec<i32>) -> i32 {
    let mut counter = 0;
    for window in nums.windows(2) {
        if window[1] > window[0] {
            counter += 1;
        };
    }
    counter
}

fn p2(nums: &Vec<i32>) -> i32 {
    let mut counter = 0;
    for window in nums.windows(4) {
        if (window[0] + window[1] + window[2]) < (window[1] + window[2] + window[3]) {
            counter += 1;
        };
    }
    counter
}
