use std::cmp;

#[aors::main]
fn main(input: &str) -> (i32, i32) {
    let input: Vec<Vec<char>> = input
        .split_whitespace()
        .map(|r| r.chars().collect())
        .collect();

    let mut p1 = 0;
    let mut all_ids: Vec<i32> = vec![];
    for v in input {
        let (row, col) = v.split_at(7);
        let row_seat = bin_search(row.to_vec(), 128);
        let col_seat = bin_search(col.to_vec(), 8);
        let seat_id = row_seat * 8 + col_seat;
        all_ids.push(seat_id);
        p1 = cmp::max(p1, seat_id);
    }

    let mut p2 = 0;
    for i in 0..1032 {
        if !all_ids.contains(&i) && all_ids.contains(&(i + 1)) && all_ids.contains(&(i - 1)) {
            p2 = i;
            break;
        }
    }

    (p1, p2)
}

fn bin_search(v: Vec<char>, length: i32) -> i32 {
    let mut from = 0;
    let mut to = length;
    for c in v {
        let size_change = (to - from) / 2;
        match c {
            'F' | 'L' => to -= size_change,
            'B' | 'R' => from += size_change,
            _ => println!("Something went wrong"),
        }
    }
    from
}
