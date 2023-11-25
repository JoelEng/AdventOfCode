use hashbrown::HashSet;

#[aors::main]
fn main(input: &str) -> (i32, i32) {
    let mut instrs = get_instr(input);

    let (p1, _) = execute(&instrs);

    for i in 0..instrs.len() {
        let instr = &mut instrs[i].0;
        let og = instr.clone();
        *instr = match instr.as_str() {
            "nop" => "jmp".to_string(),
            "jmp" => "nop".to_string(),
            _ => continue,
        };
        let (p2, completed) = execute(&instrs);
        if completed {
            return (p1, p2);
        }
        instrs[i].0 = og;
    }
    (p1, 0)
}

fn execute(instrs: &Vec<(String, i32)>) -> (i32, bool) {
    let mut executed: HashSet<i32> = HashSet::new();
    let mut next = 0;
    let mut acc = 0;
    while !executed.contains(&next) && next < instrs.len().try_into().unwrap() {
        executed.insert(next);

        let (instr, arg) = &instrs[next as usize];
        match instr.as_str() {
            "jmp" => next += arg,
            "nop" => next += 1,
            "acc" => {
                acc += arg;
                next += 1;
            }
            _ => panic!("Incorrect instruction"),
        }
    }
    (acc, next == instrs.len().try_into().unwrap())
}

fn get_instr(input: &str) -> Vec<(String, i32)> {
    input
        .lines()
        .map(|row| {
            let (instr, arg) = row.split_once(" ").unwrap();
            (instr.to_string(), arg.parse::<i32>().unwrap())
        })
        .collect()
}
