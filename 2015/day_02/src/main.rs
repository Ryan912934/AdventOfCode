struct Box {
    w: i32,
    l: i32,
    d: i32,
}

fn main() {
    let path = std::path::Path::new("./src/input.txt");
    let input = std::fs::read_to_string(path).expect("read failed");

    let lines: Vec<&str> = input.split("\n").collect();

    let mut p1 = 0;
    let mut p2 = 0;

    for l in lines {
        let split: Vec<&str> = l.split("x").collect();
        let mut mapped = split
            .into_iter()
            .map(|x| {
                let mut s = x.to_string();
                if s.ends_with('\r') {
                    s.pop();
                }
                s.parse::<i32>().expect("?")
            })
            .collect::<Vec<i32>>();
        mapped.sort();

        p1 +=
            (3 * mapped[0] * mapped[1]) + (2 * mapped[1] * mapped[2]) + (2 * mapped[0] * mapped[2]);

        p2 += (mapped[0] + mapped[0] + mapped[1] + mapped[1]) + (mapped[0] * mapped[1] * mapped[2]);
    }
    println!("Part 1: {:?}", p1);
    println!("Part 2: {:?}", p2);
}
