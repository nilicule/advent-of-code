use std::fs;

fn solve_safe_combination(filename: &str) -> i32 {
    let contents = fs::read_to_string(filename)
        .expect("Failed to read input file");

    let mut position: i32 = 50;
    let mut zero_count = 0;

    for line in contents.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let direction = line.chars().next().unwrap();
        let distance: i32 = line[1..].parse().unwrap();

        if direction == 'L' {
            position = (position - distance).rem_euclid(100);
        } else {
            position = (position + distance).rem_euclid(100);
        }

        if position == 0 {
            zero_count += 1;
        }
    }

    zero_count
}

fn main() {
    // Solve the actual puzzle
    let result = solve_safe_combination("input.txt");
    println!("\nThe actual password is: {}", result);
}
