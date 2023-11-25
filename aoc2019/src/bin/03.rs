use hashbrown::HashMap;

#[aors::main]
fn main(input: &str) -> (i32, usize) {
    let mut intersections: HashMap<(i32, i32), usize> = HashMap::new();
    let wires: Vec<&str> = input.lines().collect();
    let wire0 = wire_pos(wires[0]);
    for (pos, val) in wire_pos(wires[1]) {
        if let Some(v) = wire0.get(&pos) {
            intersections.insert(pos, v + val);
        }
    }

    let p1 = intersections
        .iter()
        .map(|((x, y), _)| x.abs() + y.abs())
        .min()
        .unwrap();

    let p2 = intersections.values().min().unwrap();
    (p1, *p2)
}

fn wire_pos(wire: &str) -> HashMap<(i32, i32), usize> {
    let mut paths: HashMap<(i32, i32), usize> = HashMap::new();
    let mut pos = (0, 0);
    let mut i = 1;
    for instr in wire.split(",") {
        let len: i32 = instr[1..].parse().unwrap();
        let dir = match &instr[..1] {
            "U" => (0, -1),
            "D" => (0, 1),
            "L" => (-1, 0),
            "R" => (1, 0),
            _ => panic!(),
        };
        for _ in 0..len {
            pos = (pos.0 + dir.0, pos.1 + dir.1);
            paths.insert(pos, i);
            i += 1;
        }
    }
    paths
}
