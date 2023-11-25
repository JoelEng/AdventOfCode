type Field = Vec<Vec<bool>>;

#[aors::main]
fn main(input: &str) -> (i64, i64) {
    let field: Field = input
        .lines()
        .map(|s| s.chars().map(|c| c == '#').collect())
        .collect();

    let p1 = check_slope(&field, 3, 1);
    let mut p2 = p1;
    p2 *= check_slope(&field, 1, 1);
    p2 *= check_slope(&field, 5, 1);
    p2 *= check_slope(&field, 7, 1);
    p2 *= check_slope(&field, 1, 2);
    (p1, p2)
}

fn check_slope(v: &Field, col_change: usize, row_change: usize) -> i64 {
    let mut hit_trees = 0;
    let v_width = v[0].len();
    let mut col = 0;
    let mut row = 0;
    while row < v.len() {
        if v[row][col] {
            hit_trees += 1
        }
        col += col_change;
        if col >= v_width {
            col -= v_width;
        }
        row += row_change;
    }
    hit_trees
}
