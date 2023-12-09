#[aors::main]
fn main(input: &str) -> (i32, i32) {
    let lines = input.lines().map(|l| {
        l.split_whitespace()
            .filter_map(|n| n.parse().ok())
            .collect::<Vec<i32>>()
    });
    let (l, r): (Vec<_>, Vec<_>) = lines.map(|l| run(&l)).unzip();
    (r.iter().sum(), l.iter().sum())
}

fn run(nums: &Vec<i32>) -> (i32, i32) {
    if nums.iter().all(|n| *n == 0) {
        return (0, 0);
    }
    let diffs: Vec<i32> = nums.windows(2).map(|w| w[1] - w[0]).collect();
    let (l, r) = run(&diffs);
    (nums[0] - l, nums[nums.len() - 1] + r)
}
