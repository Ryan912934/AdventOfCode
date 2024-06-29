use std::fs;

fn contains_three_vowels(s: &str) -> bool {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut count = 0;

    for c in s.chars() {
        if vowels.contains(&c) {
            count += 1;
            if count >= 3 {
                return true;
            }
        }
    }

    false
}

fn contains_two_in_a_row(s: &str) -> bool {
    let mut prev_char: Option<char> = None;
    for c in s.chars() {
        if let Some(prev) = prev_char {
            if prev == c {
                return true;
            }
        }
        prev_char = Some(c);
    }
    false
}

fn contains_forbidden_strings(s: &str) -> bool {
    let forbidden_strings = ["ab", "cd", "pq", "xy"];
    for forbidden in forbidden_strings.iter() {
        if s.contains(forbidden) {
            return true;
        }
    }
    false
}

fn double_double(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    for i in 1..(chars.len() - 1) {
        let pair = format!("{}{}", chars[i - 1], chars[i]);
        let pre = &s[..i - 1];
        let post = &s[i + 1..];
        if pre.contains(&pair) || post.contains(&pair) {
            return true;
        }
    }

    false
}

fn same_with_gap(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    for i in 2..chars.len() {
        if chars[i - 2] == chars[i] {
            return true;
        }
    }

    false
}
fn main() {
    let input = fs::read_to_string("./src/input.txt").expect("Failed to read file");

    let mut p1 = 0;
    let mut p2 = 0;
    for line in input.lines() {
        if contains_three_vowels(line)
            && contains_two_in_a_row(line)
            && !contains_forbidden_strings(line)
        {
            p1 += 1;
        }
        if double_double(line) && same_with_gap(line) {
            p2 += 1;
        }
    }

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}
