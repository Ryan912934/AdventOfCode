use std::fs::read_to_string;
fn main() {
    let path = std::path::Path::new("./src/input.txt");
    let input = read_to_string(path).expect("read failed");
    let mut i = 0;
    let mut idx = 0;
    let mut p2 = -1;
    for b in input.split("") {
        if b.len() == 0 {
            continue;
        }
        idx += 1;

        if *b == '('.to_string() {
            i += 1;
        } else {
            i -= 1
        }

        if i == -1 && p2 == -1 {
            p2 = idx;
        }
        // println!("{} {}", b, i);
    }

    println!("Part 1: {}", i);
    println!("Part 2: {}", p2);
}
