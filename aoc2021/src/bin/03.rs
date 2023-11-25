const WORD_SIZE: usize = 12;

#[aors::main]
fn main(input: &str) -> (i32, isize) {
    let words: Vec<String> = input.lines().map(|word| String::from(word)).collect();
    (p1(&words), p2(&words))
}

fn p1(words: &Vec<String>) -> i32 {
    let mut gamma = 0;
    let mut epsilon = 0;

    for char_index in 0..WORD_SIZE {
        if dominant_digits(&words, char_index, &vec![1; words.len()]) > 0 {
            //More ones than zeros
            gamma *= 2; //Multiplying by 2 means appedning a zero to a binary number
            gamma += 1; //Adding 1 after multiplying appends a 1 instead
            epsilon *= 2;
        } else {
            gamma *= 2;
            epsilon *= 2;
            epsilon += 1;
        }
    }

    gamma * epsilon
}

fn p2(words: &Vec<String>) -> isize {
    //keep_one defines if a one should be kept. The inversion of keep_one will be applied to zeros.
    let oxygen_keep_one = |x| x >= 0;
    let oxygen_living_numbers = solve(oxygen_keep_one, &words);

    let co2_keep_one = |x| x < 0;
    let co2_living_numbers = solve(co2_keep_one, &words);

    //Find the last surviving element
    let oxygen = words
        .iter()
        .nth(oxygen_living_numbers.iter().position(|&n| n == 1).unwrap())
        .unwrap();
    let co2 = words
        .iter()
        .nth(co2_living_numbers.iter().position(|&n| n == 1).unwrap())
        .unwrap();

    //Convert last surviving string to number
    let oxygen = isize::from_str_radix(oxygen, 2).unwrap();
    let co2 = isize::from_str_radix(co2, 2).unwrap();

    oxygen * co2
}

///Returns positive number if more ones than zeros on the specified index
pub fn dominant_digits(words: &Vec<String>, char_index: usize, living_numbers: &Vec<u32>) -> i32 {
    let mut sum = 0;

    for (word_index, word) in words.iter().enumerate() {
        match word.chars().nth(char_index) {
            Some('1') => {
                if living_numbers[word_index] == 1 {
                    sum += 1;
                }
            }
            Some('0') => {
                if living_numbers[word_index] == 1 {
                    sum -= 1;
                }
            }
            _ => println!("Something went wrong"),
        }
    }

    sum
}

pub fn solve<F>(keep_one: F, words: &Vec<String>) -> Vec<u32>
where
    F: Fn(i32) -> bool,
{
    let mut living_numbers = vec![1; words.len()];

    'outer: for char_index in 0..WORD_SIZE {
        let keep_one = keep_one(dominant_digits(words, char_index, &living_numbers));
        for (word_index, word) in words.iter().enumerate() {
            if living_numbers.iter().sum::<u32>() == 1 {
                break 'outer;
            }
            match word.chars().nth(char_index) {
                Some(n) => {
                    if living_numbers[word_index] == 1 {
                        if n == '1' && keep_one || n == '0' && !keep_one {
                            //Keep the element
                        } else {
                            living_numbers[word_index] = 0;
                        }
                    }
                }
                None => println!("Something went wrong"),
            }
        }
    }

    living_numbers
}
