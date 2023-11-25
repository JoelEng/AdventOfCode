use std::{
    cmp::{max, min},
    str::Chars,
};

#[aors::main]
fn main(input: &str) -> (u64, u64) {
    let input = input
        .chars()
        .map(|c| format!("{:0width$b}", c.to_digit(16).unwrap(), width = 4))
        .collect::<String>();
    let chars = &mut input.chars();
    let (val, ver, _) = packet(chars);

    (ver, val)
}

fn packet(chars: &mut Chars) -> (u64, u64, u64) {
    let version = take_u64(3, chars);
    let id = take_u64(3, chars);
    let (val, ver, len) = match id {
        0 => operator(|a, b| a + b, chars),
        1 => operator(|a, b| a * b, chars),
        2 => operator(|a, b| min(a, b), chars),
        3 => operator(|a, b| max(a, b), chars),
        4 => literal(chars),
        5 => operator(|a, b| if a > b { 1 } else { 0 }, chars),
        6 => operator(|a, b| if a < b { 1 } else { 0 }, chars),
        7 => operator(|a, b| if a == b { 1 } else { 0 }, chars),
        _ => (0, 0, 0),
    };
    (val, ver + version, len + 6)
}

fn operator<F: Fn(u64, u64) -> u64>(op: F, chars: &mut Chars) -> (u64, u64, u64) {
    //Match the length type ID
    let (val, ver, len) = match take_u64(1, chars) {
        0 => operator_len(op, chars),
        _ => operator_count(op, chars),
    };
    (val, ver, len + 1)
}

fn operator_len<F: Fn(u64, u64) -> u64>(op: F, chars: &mut Chars) -> (u64, u64, u64) {
    let max_len = take_u64(15, chars);
    let (mut val, mut ver, mut len) = packet(chars);
    len += 15;
    loop {
        if len - 15 == max_len {
            break;
        }
        let (sub_val, sub_ver, sub_len) = packet(chars);
        val = op(val, sub_val);
        ver += sub_ver;
        len += sub_len;
    }
    (val, ver, len)
}

fn operator_count<F: Fn(u64, u64) -> u64>(op: F, chars: &mut Chars) -> (u64, u64, u64) {
    let count = take_u64(11, chars);
    let (mut val, mut ver, mut len) = packet(chars);
    len += 11;
    for _ in 0..count - 1 {
        let (sub_val, sub_ver, sub_len) = packet(chars);
        val = op(val, sub_val);
        ver += sub_ver;
        len += sub_len;
    }
    (val, ver, len)
}

fn literal(chars: &mut Chars) -> (u64, u64, u64) {
    let (mut val, mut len) = (0, 0);
    loop {
        let last = take(1, chars) == "0";
        val *= 16;
        val += take_u64(4, chars);
        len += 5;
        if last {
            break;
        }
    }
    (val, 0, len)
}

fn take(n: usize, chars: &mut Chars) -> String {
    format!("{:0width$}", chars.take(n).collect::<String>(), width = 4)
        .trim()
        .to_string()
}

fn take_u64(n: usize, chars: &mut Chars) -> u64 {
    let s = take(n, chars);
    if s.is_empty() {
        0
    } else {
        u64::from_str_radix(&s, 2).unwrap()
    }
}
