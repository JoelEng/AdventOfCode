mod helpers;
use helpers::intcode::Intcode;

#[aors::main]
fn main(input: &str) -> (i32, i32) {
    let mut intcode = Intcode::new(input, 0);
    let mut p2 = 0;
    for n in 0..=99 {
        for v in 0..=99 {
            if run(n, v, &mut intcode) == 19690720 {
                p2 = 100 * n + v;
            }
        }
    }

    (run(12, 2, &mut intcode), p2)
}

fn run(noun: i32, verb: i32, intcode: &mut Intcode) -> i32 {
    intcode.noun_verb(noun, verb);
    intcode.run();
    intcode.first()
}
