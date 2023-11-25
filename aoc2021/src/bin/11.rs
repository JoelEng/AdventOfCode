#[aors::main]
fn main(input: &str) -> (u32, i32) {
    //Tuples consist of the value and a boolean flashed_this_turn
    let mut grid: Vec<Vec<(u8, bool)>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| (c.to_digit(10).unwrap() as u8, false))
                .collect()
        })
        .collect();
    solve(&mut grid)
}

fn solve(grid: &mut Vec<Vec<(u8, bool)>>) -> (u32, i32) {
    let mut total_flashes = 0;
    let mut has_won = false;
    let mut p1 = 0;
    let mut p2 = 0;
    for i in 1..500 {
        increase_all(grid);
        loop {
            let flashes = flash(grid);
            total_flashes += flashes;
            if flashes == 0 {
                break;
            }
        }
        if i == 100 {
            p1 = total_flashes;
        }
        if !has_won && check_part_two(&grid) {
            p2 = i;
            has_won = true;
        }
        reset_has_flashed(grid);
    }
    (p1, p2)
}

fn increase_all(grid: &mut Vec<Vec<(u8, bool)>>) {
    for row in grid {
        for octopus in row {
            octopus.0 += 1;
        }
    }
}

fn flash(grid: &mut Vec<Vec<(u8, bool)>>) -> u32 {
    let mut flashes = 0;
    let grid_len = grid.iter().len();
    for row_index in 0..grid_len {
        for oct_index in 0..grid_len {
            let value = &mut grid[row_index][oct_index];
            if value.0 > 9 {
                value.1 = true;
                value.0 = 0;
                flashes += 1;
                increase_around(grid, row_index, oct_index);
            }
        }
    }
    flashes
}

fn check_part_two(grid: &Vec<Vec<(u8, bool)>>) -> bool {
    let mut has_won = true;
    for row in grid {
        for octopus in row {
            if octopus.1 == false {
                has_won = false;
            }
        }
    }
    has_won
}

fn increase_around(grid: &mut Vec<Vec<(u8, bool)>>, row_index: usize, oct_index: usize) {
    if row_index > 0 && !grid[row_index - 1][oct_index].1 {
        grid[row_index - 1][oct_index].0 += 1;
    }
    if oct_index > 0 && !grid[row_index][oct_index - 1].1 {
        grid[row_index][oct_index - 1].0 += 1;
    }
    if row_index < grid.len() - 1 && !grid[row_index + 1][oct_index].1 {
        grid[row_index + 1][oct_index].0 += 1;
    }
    if oct_index < grid.len() - 1 && !grid[row_index][oct_index + 1].1 {
        grid[row_index][oct_index + 1].0 += 1;
    }

    if row_index > 0 && oct_index > 0 && !grid[row_index - 1][oct_index - 1].1 {
        grid[row_index - 1][oct_index - 1].0 += 1;
    }
    if row_index > 0 && oct_index < grid.len() - 1 && !grid[row_index - 1][oct_index + 1].1 {
        grid[row_index - 1][oct_index + 1].0 += 1;
    }
    if row_index < grid.len() - 1 && oct_index > 0 && !grid[row_index + 1][oct_index - 1].1 {
        grid[row_index + 1][oct_index - 1].0 += 1;
    }
    if row_index < grid.len() - 1
        && oct_index < grid.len() - 1
        && !grid[row_index + 1][oct_index + 1].1
    {
        grid[row_index + 1][oct_index + 1].0 += 1;
    }
}

fn reset_has_flashed(grid: &mut Vec<Vec<(u8, bool)>>) {
    let grid_len = grid.iter().len();
    for row_index in 0..grid_len {
        for oct_index in 0..grid_len {
            grid[row_index][oct_index].1 = false;
        }
    }
}
