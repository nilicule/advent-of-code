use std::fs;
use std::collections::HashSet;

fn count_accessible_rolls(grid: &Vec<Vec<char>>) -> usize {
    let rows = grid.len();
    if rows == 0 {
        return 0;
    }
    let cols = grid[0].len();

    // All 8 directions: up, down, left, right, and 4 diagonals
    let directions: [(i32, i32); 8] = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),           (0, 1),
        (1, -1),  (1, 0),  (1, 1),
    ];

    let mut accessible_count = 0;

    for i in 0..rows {
        for j in 0..cols {
            // Only check positions with paper rolls
            if grid[i][j] == '@' {
                // Count adjacent paper rolls
                let mut adjacent_rolls = 0;
                for &(di, dj) in &directions {
                    let ni = i as i32 + di;
                    let nj = j as i32 + dj;
                    // Check if in bounds and is a paper roll
                    if ni >= 0 && ni < rows as i32 && nj >= 0 && nj < cols as i32 {
                        if grid[ni as usize][nj as usize] == '@' {
                            adjacent_rolls += 1;
                        }
                    }
                }

                // Accessible if fewer than 4 adjacent rolls
                if adjacent_rolls < 4 {
                    accessible_count += 1;
                }
            }
        }
    }

    accessible_count
}

fn find_accessible_positions(grid: &Vec<Vec<char>>) -> HashSet<(usize, usize)> {
    let rows = grid.len();
    if rows == 0 {
        return HashSet::new();
    }
    let cols = grid[0].len();

    let directions: [(i32, i32); 8] = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),           (0, 1),
        (1, -1),  (1, 0),  (1, 1),
    ];

    let mut accessible = HashSet::new();

    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == '@' {
                let mut adjacent_rolls = 0;
                for &(di, dj) in &directions {
                    let ni = i as i32 + di;
                    let nj = j as i32 + dj;
                    if ni >= 0 && ni < rows as i32 && nj >= 0 && nj < cols as i32 {
                        if grid[ni as usize][nj as usize] == '@' {
                            adjacent_rolls += 1;
                        }
                    }
                }

                if adjacent_rolls < 4 {
                    accessible.insert((i, j));
                }
            }
        }
    }

    accessible
}

fn remove_accessible_rolls(grid_str: &Vec<String>) -> usize {
    // Convert to mutable grid
    let mut grid: Vec<Vec<char>> = grid_str
        .iter()
        .map(|s| s.chars().collect())
        .collect();

    let mut total_removed = 0;

    loop {
        // Find all accessible positions
        let accessible = find_accessible_positions(&grid);

        if accessible.is_empty() {
            break;
        }

        // Remove all accessible rolls
        for (i, j) in &accessible {
            grid[*i][*j] = '.';
        }

        total_removed += accessible.len();
    }

    total_removed
}

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Failed to read input.txt");

    let grid: Vec<String> = input
        .lines()
        .map(|line| line.to_string())
        .collect();

    // Part 1
    let grid_chars: Vec<Vec<char>> = grid
        .iter()
        .map(|s| s.chars().collect())
        .collect();
    let result_part1 = count_accessible_rolls(&grid_chars);
    println!("Part 1: {}", result_part1);

    // Part 2
    let result_part2 = remove_accessible_rolls(&grid);
    println!("Part 2: {}", result_part2);
}
