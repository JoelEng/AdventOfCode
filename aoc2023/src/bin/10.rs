use hashbrown::HashMap;
use itertools::Itertools;

#[aors::main]
fn main(input: &str) -> (i32, i32) {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
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
    let dir = (phi(-1, 1), 0);
    let p1 = scurry((row as i32 + dir.0, col as i32 + dir.1), dir, &pipes, &grid).unwrap();
    ((p1 + 1) / 2, 0)
}

fn scurry(
    pos: (i32, i32),
    dir: (i32, i32),
    pipes: &HashMap<char, (i32, i32)>,
    grid: &Vec<Vec<char>>,
) -> Option<i32> {
    let a = grid.get(pos.0 as usize)?.get(pos.1 as usize)?;
    if *a == 'S' {
        return None;
    }
    let pipe = pipes.get(a)?;
    let new_dir = (dir.0 + pipe.0, dir.1 + pipe.1);
    let new_pos = (pos.0 + new_dir.0, pos.1 + new_dir.1);
    Some(1 + scurry(new_pos, new_dir, pipes, grid).unwrap_or(0))
}
