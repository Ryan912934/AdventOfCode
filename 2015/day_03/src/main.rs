use std::collections::HashMap;

fn update_map(m: &mut HashMap<String, i32>, x: &mut i32, y: &mut i32, dx: i32, dy: i32) {
    *x += dx;
    *y += dy;
    let key = format!("{},{}", x, y);
    if m.contains_key(&key) {
        let cur = m.get(&key).unwrap();
        m.insert(key, cur + 1);
    } else {
        m.insert(key, 1);
    }
}
fn main() {
    let input =
        std::fs::read_to_string(std::path::Path::new("./src/input.txt")).expect("err reading");

    let mut cx = 0;
    let mut cy = 0;

    let mut locs = HashMap::new();

    locs.insert(format!("{},{}", 0, 0), 1);

    let mut p2 = locs.clone();
    let mut x1 = 0;
    let mut x2 = 0;
    let mut y1 = 0;
    let mut y2 = 0;

    let mut robo = true;

    for m in input.split("") {
        if m.len() == 0 {
            continue;
        }
        let mut dx = 0;
        let mut dy = 0;
        match m {
            ">" => dx += 1,
            "<" => dx -= 1,
            "^" => dy += 1,
            "v" => dy -= 1,
            _ => (),
        }

        update_map(&mut locs, &mut cx, &mut cy, dx, dy);

        if robo {
            update_map(&mut p2, &mut x1, &mut y1, dx, dy);
        } else {
            update_map(&mut p2, &mut x2, &mut y2, dx, dy);
        }
        robo = !robo;
    }

    let total = locs.keys().len();
    let total2 = p2.keys().len();

    println!("Part 1: {}", total);
    println!("Part 1: {}", total2);
}
