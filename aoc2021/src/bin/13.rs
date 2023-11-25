use std::io;
use std::io::Write; // <--- bring flush() into scope

const MATRIX_SIZE: usize = 1500;

#[aors::main]
fn main(input: &str) -> (u32, &str) {
    let (mut matrix, folds) = get_input(input);
    let mut first_time = true;
    let mut p1 = 0;

    for (dir, index) in folds {
        if dir == 'x' {
            matrix = fold_x(index, &mut matrix);
        } else {
            matrix = fold_y(index, &mut matrix);
        }
        if first_time {
            p1 = count_dots(&matrix);
            first_time = false;
        }
    }
    print_matrix(&matrix);

    (p1, "")
}

fn get_input(input: &str) -> (Vec<Vec<bool>>, Vec<(char, usize)>) {
    let mut matrix = vec![vec![false; MATRIX_SIZE]; MATRIX_SIZE];
    let mut folds = vec![];

    for line in input.lines().filter(|line| !line.is_empty()) {
        if line.chars().next().unwrap() == 'f' {
            let mut split = line.split('=');
            let dir = split.next().unwrap().chars().last().unwrap();
            let split_index = split.next().unwrap().parse::<usize>().unwrap();
            folds.push((dir, split_index));
        } else {
            let mut split = line.split(',');
            let x = split.next().unwrap().parse::<usize>().unwrap();
            let y = split.next().unwrap().parse::<usize>().unwrap();
            matrix[y][x] = true;
        }
    }

    (matrix, folds)
}

fn fold_x(fold_index: usize, matrix: &mut Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    for row in 0..matrix.len() {
        for col in 0..fold_index {
            let dist = fold_index - col;
            matrix[row][col] = matrix[row][col] || matrix[row][col + 2 * dist];
        }
        matrix[row] = matrix[row][0..fold_index].to_vec();
    }
    matrix.to_vec()
}

fn fold_y(fold_index: usize, matrix: &mut Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    for row in 0..fold_index {
        let dist = fold_index - row;
        for col in 0..matrix[0].len() {
            matrix[row][col] = matrix[row][col] || matrix[row + 2 * dist][col];
        }
    }
    matrix[0..fold_index].to_vec()
}

fn print_matrix(matrix: &Vec<Vec<bool>>) {
    for row in matrix {
        for elem in row {
            if *elem {
                print!("██");
            } else {
                print!("  ");
            }
            io::stdout().flush().unwrap();
        }
        println!("");
    }
}

fn count_dots(matrix: &Vec<Vec<bool>>) -> u32 {
    let mut dots = 0u32;
    for row in matrix {
        for elem in row {
            if *elem {
                dots += 1;
            }
        }
    }
    dots
}
