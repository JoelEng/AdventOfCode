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
    let (mut galaxies, mut p1, mut p2) = (vec![], 0, 0);
    for y in 0..grid.len() {
        for x in 0..grid.len() {
            if grid[y][x] == '#' {
                for (oy, ox) in &galaxies {
                    let (ry, rx) = (y.min(*oy)..y.max(*oy), x.min(*ox)..x.max(*ox));
                    let ey = rows.iter().filter(|r| ry.contains(r)).count() as i64;
                    let ex = cols.iter().filter(|c| rx.contains(c)).count() as i64;
                    let diff = (y as i64 - *oy as i64).abs() + (x as i64 - *ox as i64).abs();
                    p1 += diff + ey + ex;
                    p2 += diff + (1000000 - 1) * (ey + ex);
                }
                galaxies.push((y, x));
            }
        }
    }
    (p1, p2)
}
