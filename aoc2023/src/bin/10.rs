use itertools::Itertools;
use phf::{phf_map, Map};

static PIPES: Map<char, (i32, i32)> = phf_map! {
    '|' => (0, 0),
    '-' => (0, 0),
    'L' => (-1, 1),
    'J' => (-1,-1),
    '7' => (1,-1),
    'F' => (1,1),
};

#[aors::main]
fn main(input: &str) -> (i32, usize) {
    let mut grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let (row, line) = input.lines().find_position(|l| l.contains("S")).unwrap();
    let col = line.chars().position(|c| c == 'S').unwrap();
    grid[row][col] = phi('J', 'F');
    let dir = phi((1, 0), (-1, 0));
    let p1 = scurry((row as i32, col as i32), dir, &mut grid).unwrap();
    (p1 / 2, p2(&grid))
}

fn scurry(pos: (i32, i32), dir: (i32, i32), grid: &mut Vec<Vec<char>>) -> Option<i32> {
    let a = &mut grid[pos.0 as usize][pos.1 as usize];
    let pipe = PIPES.get(a)?;
    *a = match ['|', 'L', 'J'].contains(a) {
        true => '!',
        false => '_',
    };
    let ndir = (dir.0 + pipe.0, dir.1 + pipe.1);
    Some(1 + scurry((pos.0 + ndir.0, pos.1 + ndir.1), ndir, grid).unwrap_or(1))
}

fn p2(grid: &Vec<Vec<char>>) -> usize {
    grid.iter()
        .map(|r| {
            r.iter()
                .enumerate()
                .filter(|(_, c)| !['!', '_'].contains(c))
                .filter(|(i, _)| r[..*i].iter().filter(|c| *c == &'!').count() % 2 == 1)
                .count()
        })
        .sum()
}
