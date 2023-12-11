use num::abs;

#[aors::main]
fn main(input: &str) -> (i64, i64) {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let (mut rows, mut cols) = (vec![], vec![]);
    for i in 0..grid.len() {
        if grid.iter().all(|l| l[i] != '#') {
            cols.push(i);
        }
        if grid[i].iter().all(|c| *c != '#') {
            rows.push(i);
        }
    }
    let mut galaxies = vec![];
    let (mut p1, mut p2) = (0, 0);
    for i in 0..grid.len() {
        for j in 0..grid.len() {
            if grid[i][j] == '#' {
                for (oi, oj) in &galaxies {
                    let exp_y = rows
                        .iter()
                        .filter(|r| (i.min(*oi)..i.max(*oi)).contains(r))
                        .count() as i64;
                    let exp_x = cols
                        .iter()
                        .filter(|c| (j.min(*oj)..j.max(*oj)).contains(c))
                        .count() as i64;
                    let steps_y = abs(i as i64 - *oi as i64);
                    let steps_x = abs(j as i64 - *oj as i64);
                    p1 += steps_y + steps_x + exp_y + exp_x;
                    p2 += steps_y + steps_x + (1000000 - 1) * (exp_y + exp_x);
                }
                galaxies.push((i, j));
            }
        }
    }
    (p1, p2)
}
