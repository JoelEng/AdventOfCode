use hashbrown::HashMap;

type Seats = Vec<Vec<char>>;
type Seat = (i32, i32);
type Adjacents = HashMap<Seat, HashMap<(i32, i32), Seat>>;

const DIRS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[aors::main]
fn main(input: &str) -> (u32, u32) {
    let seats: Seats = input.lines().map(|s| s.chars().collect()).collect();
    (iter(&seats, false), iter(&seats, true))
}

fn iter(seats: &Seats, p2: bool) -> u32 {
    let adjacents_p2 = find_adjacents(&seats, p2);
    let seats_p2 = round(seats, true, &adjacents_p2);
    sum_seats(seats_p2)
}

fn sum_seats(s: Seats) -> u32 {
    s.iter()
        .map(|r| r.iter().map(|s| if *s == '#' { 1 } else { 0 }).sum::<u32>())
        .sum()
}

fn round(seats: &Seats, p2: bool, adjacents: &Adjacents) -> Seats {
    let mut new_seats: Seats = seats.clone();
    for i in 0..seats.len() {
        for j in 0..seats[0].len() {
            let new_seat = &mut new_seats[i][j];
            let count_adj = count_adjacent(i as i32, j as i32, &seats, &adjacents);
            match new_seat {
                'L' => {
                    if count_adj == 0 {
                        *new_seat = '#';
                    }
                }
                '#' => {
                    if (count_adj >= 4 && !p2) || count_adj >= 5 {
                        *new_seat = 'L';
                    }
                }
                '.' => (),
                _ => unreachable!(),
            };
        }
    }
    if new_seats == *seats {
        return new_seats;
    } else {
        return round(&new_seats, p2, adjacents);
    }
}

fn find_adjacents(seats: &Seats, p2: bool) -> Adjacents {
    let mut adjacents: Adjacents = HashMap::new();
    let height: i32 = seats.len().try_into().unwrap();
    let width: i32 = seats[0].len().try_into().unwrap();
    for row in 0..height {
        for col in 0..width {
            let seat: Seat = (row, col);
            'outer: for (i, j) in DIRS {
                let (mut r_offset, mut c_offset) = (i, j);
                let (mut r, mut c): (i32, i32);
                loop {
                    (r, c) = ((row as i32) + r_offset, (col as i32) + c_offset);
                    if r < 0 || c < 0 || r >= height || c >= width {
                        continue 'outer;
                    }
                    if seats[r as usize][c as usize] != '.' || !p2 {
                        break;
                    }
                    r_offset += i;
                    c_offset += j;
                }
                adjacents.entry(seat).or_default().insert((i, j), (r, c));
            }
        }
    }
    adjacents
}

fn count_adjacent(row: i32, col: i32, seats: &Seats, adjacents: &Adjacents) -> u32 {
    let mut count = 0;
    for (i, j) in adjacents.get(&(row, col)).unwrap().values() {
        count += if seats[*i as usize][*j as usize] == '#' {
            1
        } else {
            0
        };
    }
    count
}
