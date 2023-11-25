mod helpers;
use helpers::intcode::Intcode;

#[aors::main]
fn main(input: &str) -> (i32, i32) {
    (Intcode::new(input, 1).run(), Intcode::new(input, 5).run())
}
