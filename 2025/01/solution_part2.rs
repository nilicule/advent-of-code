use std::fs;

fn solve_safe_combination_with_clicks(filename: &str) -> i32 {
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

        // Simulate the rotation click by click
        let mut current = position;
        for _ in 0..distance {
            if direction == 'R' {
                current = (current + 1).rem_euclid(100);
            } else {
                current = (current - 1).rem_euclid(100);
            }

            if current == 0 {
                zero_count += 1;
            }
        }

        position = current;
    }

    zero_count
}

fn main() {
    // Solve the actual puzzle
    let result = solve_safe_combination_with_clicks("input.txt");
    println!("\nThe actual password (method 0x434C49434B) is: {}", result);
}
