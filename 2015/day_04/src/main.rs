use md5;

fn find_md5_with_prefix(prefix: &str, key: &str) -> i32 {
    let mut x = 0;
    while !format!("{:x}", md5::compute(format!("{}{}", key, x))).starts_with(prefix) {
        x += 1;
    }

    x
}

fn main() {
    let input = std::fs::read_to_string(std::path::Path::new("./src/input.txt")).expect("err");
    let p1 = find_md5_with_prefix("00000", &input);
    println!("Part 1: {}", p1);
    let p2 = find_md5_with_prefix("000000", &input);
    println!("Part 2: {}", p2);
}
