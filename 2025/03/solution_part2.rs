use std::fs;

fn find_max_joltage_part2(bank: &str, num_batteries: usize) -> u64 {
    // Find the maximum joltage by picking exactly num_batteries batteries.
    //
    // Strategy: Use a greedy approach. At each step, pick the largest digit
    // from the remaining candidates that still leaves enough digits to pick
    // the required total.
    //
    // For example, if we need to pick 12 digits from a 15-digit bank:
    // - We can skip at most 3 digits
    // - At position i, we look ahead and find the largest digit in the window
    //   that ensures we can still pick enough remaining digits

    let chars: Vec<char> = bank.chars().collect();
    let n = chars.len();

    if n < num_batteries {
        // Not enough batteries in the bank
        return 0;
    }

    let mut result = String::new();
    let mut start = 0; // Current position to search from

    for picked in 0..num_batteries {
        // How many more batteries do we need after this one?
        let remaining_needed = num_batteries - picked - 1;

        // We can search up to position where we still have enough batteries left
        // to pick the remaining ones
        let search_end = n - remaining_needed;

        // Find the maximum digit in the valid search range
        let mut max_digit = '0';
        let mut max_pos = start;

        for i in start..search_end {
            if chars[i] > max_digit {
                max_digit = chars[i];
                max_pos = i;
            }
        }

        result.push(max_digit);
        start = max_pos + 1; // Next search starts after this position
    }

    result.parse().unwrap_or(0)
}

fn solve_part2(input_file: &str) -> u64 {
    // Solve Day 3 Part 2: Maximum joltage from battery banks (12 batteries each).
    let contents = fs::read_to_string(input_file)
        .expect("Failed to read input file");

    let banks: Vec<&str> = contents
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect();

    let mut total: u64 = 0;
    for bank in banks {
        let max_joltage = find_max_joltage_part2(bank, 12);
        total += max_joltage;
    }

    total
}

fn main() {
    // Test with examples first
    let test_cases = vec![
        ("987654321111111", 987654321111u64),
        ("811111111111119", 811111111119u64),
        ("234234234234278", 434234234278u64),
        ("818181911112111", 888911112111u64),
    ];

    println!("Testing examples:");
    let mut all_correct = true;
    for (bank, expected) in &test_cases {
        let result = find_max_joltage_part2(bank, 12);
        let status = if result == *expected { "✓" } else { "✗" };
        println!("{} {}: got {}, expected {}", status, bank, result, expected);
        if result != *expected {
            all_correct = false;
        }
    }

    if all_correct {
        println!("\nAll tests passed!");
        let expected_total: u64 = test_cases.iter().map(|(_, expected)| expected).sum();
        println!("Expected total from examples: {}", expected_total);

        println!("\nRunning on actual input...");
        let result = solve_part2("/Users/remco/PycharmProjects/adventOfCode/2025/03/input.txt");
        println!("Total output joltage: {}", result);
    } else {
        println!("\nTests failed, not running on actual input.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        assert_eq!(find_max_joltage_part2("987654321111111", 12), 987654321111);
        assert_eq!(find_max_joltage_part2("811111111111119", 12), 811111111119);
        assert_eq!(find_max_joltage_part2("234234234234278", 12), 434234234278);
        assert_eq!(find_max_joltage_part2("818181911112111", 12), 888911112111);
    }

    #[test]
    fn test_total() {
        let test_cases = vec![
            "987654321111111",
            "811111111111119",
            "234234234234278",
            "818181911112111",
        ];
        let total: u64 = test_cases
            .iter()
            .map(|&bank| find_max_joltage_part2(bank, 12))
            .sum();
        assert_eq!(total, 3121910778619);
    }
}
