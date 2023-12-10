use hashbrown::HashMap;
use itertools::Itertools;

#[aors::main]
fn main(input: &str) -> (i32, u32) {
    let mut grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let (row, line) = input.lines().find_position(|l| l.contains("S")).unwrap();
    let col = line.chars().position(|c| c == 'S').unwrap();
    let pipes: HashMap<char, (i32, i32)> = HashMap::from([
        ('|', (0, 0)),
        ('-', (0, 0)),
        ('L', (-1, 1)),
        ('J', (-1, -1)),
        ('7', (1, -1)),
        ('F', (1, 1)),
    ]);
    *grid.get_mut(row).unwrap().get_mut(col).unwrap() = phi('J', 'F');
    let dir = phi((-1, 0), (1, 0));
    let p1 = scurry(
        (row as i32, col as i32),
        (row as i32 + dir.0, col as i32 + dir.1),
        dir,
        &pipes,
        &mut grid,
    )
    .unwrap();
    (p1 / 2, p2(&grid))
}

fn scurry(
    start: (i32, i32),
    pos: (i32, i32),
    dir: (i32, i32),
    pipes: &HashMap<char, (i32, i32)>,
    grid: &mut Vec<Vec<char>>,
) -> Option<i32> {
    let a = grid.get_mut(pos.0 as usize)?.get_mut(pos.1 as usize)?;
    let pipe = pipes.get(a)?;
    *a = match ['|', 'L', 'J'].contains(a) {
        true => '!',
        false => '_',
    };
    if pos == start {
        return Some(1);
    }
    let new_dir = (dir.0 + pipe.0, dir.1 + pipe.1);
    let new_pos = (pos.0 + new_dir.0, pos.1 + new_dir.1);
    Some(1 + scurry(start, new_pos, new_dir, pipes, grid).unwrap_or(0))
}

fn p2(grid: &Vec<Vec<char>>) -> u32 {
    let mut count = 0;
    for r in grid {
        for (i, c) in r.iter().enumerate() {
            if !['!', '_'].contains(c) {
                if r[..i].iter().filter(|c| *c == &'!').count() % 2 == 1 {
                    count += 1;
                }
            }
        }
    }
    count
}
