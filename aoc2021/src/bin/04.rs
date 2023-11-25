const BOARD_SIZE: usize = 5;
use itertools::Itertools;

#[aors::main]
fn main(input: &str) -> (usize, usize) {
    let (draws, mut boards) = get_input(input);
    (p1(&draws, &mut boards), p2(&draws, &mut boards))
}

fn get_input(input: &str) -> (Vec<u8>, Vec<Board>) {
    let mut input = input.lines().filter(|&x| !x.is_empty());

    //Split drawn numbers
    let draws = input.next().unwrap().split(',');
    //Parse drawn numbers to u8
    let draws: Vec<u8> = draws
        .map(|x| x.parse().expect("Something went wrong"))
        .collect();

    //Make an array of Board structs
    let boards: Vec<Board> = input
        .chunks(BOARD_SIZE)
        .into_iter()
        .map(|board| {
            Board::new(
                board
                    .map(|row| {
                        row.split_whitespace()
                            .map(|num| num.parse().unwrap())
                            .collect()
                    })
                    .collect(),
            )
        })
        .collect(); //There has to be a better way to do this

    (draws, boards)
}

fn p1(draws: &Vec<u8>, boards: &mut Vec<Board>) -> usize {
    for draw in draws {
        for board in &mut *boards {
            let possible_score = board.fill_in(&draw);
            if possible_score != 0 {
                return possible_score;
            }
        }
    }
    0
}

fn p2(draws: &Vec<u8>, boards: &mut Vec<Board>) -> usize {
    let mut last_score = 0;

    for draw in draws {
        for board in &mut *boards {
            let board_score = board.fill_in(&draw);
            if board_score != 0 {
                last_score = board_score;
            }
        }
    }
    last_score
}

pub struct Board {
    values: Vec<Vec<u8>>,
    filled_in: [[bool; BOARD_SIZE]; BOARD_SIZE],
    filled_in_count: u8,
    has_won: bool,
}

impl Board {
    pub fn new(values: Vec<Vec<u8>>) -> Board {
        Board {
            values,
            filled_in: [[false; BOARD_SIZE]; BOARD_SIZE],
            filled_in_count: 0,
            has_won: false,
        }
    }

    pub fn fill_in_index(&mut self, index: usize) {
        self.filled_in[index / BOARD_SIZE][index % BOARD_SIZE] = true;
        self.filled_in_count += 1;
    }

    pub fn fill_in(&mut self, num: &u8) -> usize {
        let mut current_score = 0;
        let mut has_won = false;
        for (index, sqr) in self.values.clone().iter().flatten().enumerate() {
            if sqr == num {
                self.fill_in_index(index);
                let possible_score = self.check_win(index, *sqr);
                if !has_won && possible_score != 0 {
                    current_score = possible_score;
                    has_won = true;
                }
            }
        }
        current_score
    }

    pub fn check_win(&mut self, sqr_index: usize, square: u8) -> usize {
        let mut row_won = true;
        let mut col_won = true;
        for i in 0..BOARD_SIZE {
            if !self.filled_in[i][sqr_index % BOARD_SIZE] {
                row_won = false;
            }
            if !self.filled_in[sqr_index / BOARD_SIZE][i] {
                col_won = false;
            }
        }
        let won = row_won || col_won;

        if won && !self.has_won {
            self.has_won = true;
            return self.calc_points(square);
        }

        return 0;
    }

    pub fn calc_points(&self, square: u8) -> usize {
        let mut sum = 0;
        let values = self.values.iter().flatten();

        for (index, sqr) in values.enumerate() {
            if !self.filled_in[index / BOARD_SIZE][index % BOARD_SIZE] {
                sum += *sqr as u32;
            }
        }

        sum as usize * square as usize
    }
}
