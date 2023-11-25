use super::get_digit;

#[derive(Debug)]
pub enum Mode {
    Position,
    Immediate,
}

pub struct Intcode {
    code: Vec<i32>,
    init_code: Vec<i32>,
    pointer: usize,
    input: i32,
    output: i32,
    first: i32,
}

impl Intcode {
    pub fn new(code: &str, input: i32) -> Intcode {
        let code: Vec<i32> = code.split(",").filter_map(|s| s.parse().ok()).collect();
        Intcode {
            init_code: code.clone(),
            code,
            pointer: 0,
            input,
            output: 0,
            first: 0,
        }
    }

    /** Execute the Intcode program, then reset it*/
    pub fn run(&mut self) -> i32 {
        while self.instruction() {}
        self.first = self.code[0];
        self.code = self.init_code.clone();
        self.pointer = 0;
        self.output
    }

    pub fn noun_verb(&mut self, noun: i32, verb: i32) {
        self.code[1] = noun;
        self.code[2] = verb;
    }

    /** Returns the first value in the code. Used for Day 02 */
    pub fn first(&self) -> i32 {
        self.first
    }

    /** Returns the value of a parameter based on its Mode */
    fn param(&self, param: u32) -> i32 {
        let pos = self.pointer + param as usize;
        match self.get_mode(self.pointer, param) {
            Mode::Position => self.code[self.code[pos] as usize],
            Mode::Immediate => self.code[pos],
        }
    }

    /** Updates the value at the position found in code\[self.pointer + param\]. Does not update the value at code\[pos\]
     */
    fn set(&mut self, param: usize, val: i32) {
        let new_pos = self.code[self.pointer + param] as usize;
        self.code[new_pos] = val;
    }

    fn get_mode(&self, op_pos: usize, param: u32) -> Mode {
        match get_digit(self.code[op_pos] as u32, param + 1, 1) {
            0 => Mode::Position,
            1 => Mode::Immediate,
            _ => panic!("Wrong mode digit"),
        }
    }

    fn instruction(&mut self) -> bool {
        let pos = self.pointer;
        let opcode = get_digit(self.code[pos] as u32, 0, 2);
        match opcode {
            1 => self.set(3, self.param(1) + self.param(2)),
            2 => self.set(3, self.param(1) * self.param(2)),
            3 => self.set(1, self.input),
            4 => {
                if self.output != 0 {
                    panic!("Program failed with output {}", self.output);
                } else {
                    self.output = self.param(1);
                }
            }
            5 => {
                if self.param(1) != 0 {
                    self.pointer = self.param(2) as usize;
                }
            }
            6 => {
                if self.param(1) == 0 {
                    self.pointer = self.param(2) as usize;
                }
            }
            7 => {
                if self.param(1) < self.param(2) {
                    self.set(3, 1);
                } else {
                    self.set(3, 0)
                }
            }
            8 => {
                if self.param(1) == self.param(2) {
                    self.set(3, 1);
                } else {
                    self.set(3, 0)
                }
            }
            _ => (),
        }
        let param_count = match opcode {
            1 => 3,
            2 => 3,
            3 => 1,
            4 => 1,
            5 => 2,
            6 => 2,
            7 => 3,
            8 => 3,
            99 | _ => return false,
        };
        if self.pointer == pos {
            self.pointer += param_count + 1;
        }
        true
    }
}
