use std::fs;

fn do_action(
    toggle: bool,
    on: bool,
    parts: Vec<&str>,
    light1: &mut Vec<Vec<bool>>,
    lights2: &mut Vec<Vec<i32>>,
) {
    let startCords: Vec<&str>;
    let endCords: Vec<&str>;
    if (toggle) {
        startCords = parts[1].split(',').collect::<Vec<&str>>();
        endCords = parts[3].split(',').collect::<Vec<&str>>();
    } else {
        startCords = parts[2].split(',').collect::<Vec<&str>>();
        endCords = parts[4].split(',').collect::<Vec<&str>>();
    }
    let start = startCords[0].parse::<usize>().unwrap();
    let end = startCords[1].parse::<usize>().unwrap();
    let end_x = endCords[0].parse::<usize>().unwrap();
    let end_y = endCords[1].parse::<usize>().unwrap();

    for i in start..end_x + 1 {
        for j in end..end_y + 1 {
            if (toggle) {
                light1[i][j] = !light1[i][j];
                lights2[i][j] += 2
            } else if (on) {
                light1[i][j] = true;
                lights2[i][j] += 1
            } else {
                light1[i][j] = false;
                lights2[i][j] -= 1;
                if (lights2[i][j] < 0) {
                    lights2[i][j] = 0;
                }
            }
        }
    }
}
fn main() {
    let file_contents = fs::read_to_string("./src/input.txt").expect("Failed to read file");

    let mut lights1 = vec![vec![false; 1000]; 1000];
    let mut lights2 = vec![vec![0; 1000]; 1000];
    for line in file_contents.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let action = parts[0];

        // Perform the desired action based on the parsed values
        match action {
            "turn" => {
                let sub_action = parts[1];
                match sub_action {
                    "on" => {
                        do_action(false, true, parts, &mut lights1, &mut lights2);
                    }
                    "off" => {
                        do_action(false, false, parts, &mut lights1, &mut lights2);
                    }
                    _ => {
                        println!("err")
                    }
                }
            }
            _ => {
                // Invalid action
                do_action(true, false, parts, &mut lights1, &mut lights2);
            }
        }
    }

    let count = lights1.iter().flatten().filter(|&&light| light).count();
    println!("Number of true lights: {}", count);
    let count2 = lights2.iter().flatten().sum::<i32>();
    println!("Total brightness: {}", count2);
}
