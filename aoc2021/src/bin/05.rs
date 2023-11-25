use std::cmp::{max, min};

const BOARD_SIZE: usize = 1000;
type Board = [[u32; BOARD_SIZE]; BOARD_SIZE];

#[aors::main]
fn main(input: &str) -> (i32, i32) {
    let vents = input
        .lines()
        .map(|line| {
            let mut line = line.split_whitespace();
            let first = create_tuple(line.next());
            let second = create_tuple(line.last());
            Vent::new(first, second)
        })
        .collect::<Vec<Vent>>();
    (p1(&vents), p2(&vents))
}

fn p1(vents: &Vec<Vent>) -> i32 {
    solve(vents, false)
}

fn p2(vents: &Vec<Vent>) -> i32 {
    solve(vents, true)
}

///Helper function for get_input
fn create_tuple(input: Option<&str>) -> (usize, usize) {
    let mut elems = input.unwrap().trim().split(',');
    (
        elems
            .next()
            .unwrap()
            .parse::<usize>()
            .expect("Something went wrong"),
        elems
            .next()
            .unwrap()
            .parse::<usize>()
            .expect("Something went wrong"),
    )
}

fn solve(vents: &Vec<Vent>, is_part_two: bool) -> i32 {
    let mut board: Board = [[0; BOARD_SIZE]; BOARD_SIZE];

    for vent in vents {
        vent.add_to(&mut board, is_part_two);
    }

    let mut count = 0;
    for sqr in board.iter().flatten() {
        if *sqr >= 2 {
            count += 1;
        }
    }
    count
}

pub struct Vent {
    start: (usize, usize),
    end: (usize, usize),
}

impl Vent {
    pub fn new(start: (usize, usize), end: (usize, usize)) -> Vent {
        Vent { start, end }
    }

    pub fn add_to(&self, board: &mut Board, is_part_two: bool) {
        let x1 = self.start.0;
        let y1 = self.start.1;

        let x2 = self.end.0;
        let y2 = self.end.1;

        if x1 == x2 {
            let add_to_board = |y: usize| board[x1][y] += 1;
            Vent::helper(y1, y2, add_to_board);
        } else if y1 == y2 {
            let add_to_board = |x: usize| board[x][y1] += 1;
            Vent::helper(x1, x2, add_to_board);
        } else if (x2 as isize - x1 as isize).abs() == (y2 as isize - y1 as isize).abs()
            && is_part_two
        {
            let mut add_to_board = |x: usize, y: usize| board[x][y] += 1;
            let x_start = min(x1, x2);
            let x_end = max(x1, x2);
            let y_start = if x_start == x1 { y1 } else { y2 };
            let y_end = if !(x_start == x1) { y1 } else { y2 };
            //let x_range = x_start..=x_end;
            let len = x_end - x_start;

            if y_start < y_end {
                for l in 0..=len {
                    add_to_board(x_start + l, y_start + l);
                }
            } else {
                for l in 0..=len {
                    add_to_board(x_start + l, (y_start as isize - l as isize) as usize);
                }
            }
        }
    }

    pub fn helper<F>(a1: usize, a2: usize, mut add_to_board: F)
    where
        F: FnMut(usize) -> (),
    {
        let start = min(a1, a2);
        let end = max(a1, a2);
        for y in start..=end {
            add_to_board(y);
        }
    }
}
