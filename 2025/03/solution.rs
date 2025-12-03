use std::fs;

fn find_max_joltage(bank: &str) -> u32 {
    /// Find the maximum joltage possible from a bank of batteries.
    ///
    /// We need to pick exactly two batteries at positions i and j (where i < j)
    /// and concatenate them to form the largest possible number. We cannot
    /// rearrange the batteries, so we try all possible pairs.
    let chars: Vec<char> = bank.chars().collect();
    let n = chars.len();
    let mut max_joltage = 0;

    // Try all pairs (i, j) where i < j
    for i in 0..n {
        for j in (i + 1)..n {
            // Form the number by concatenating digits at positions i and j
            let joltage_str = format!("{}{}", chars[i], chars[j]);
            let joltage: u32 = joltage_str.parse().unwrap_or(0);
            max_joltage = max_joltage.max(joltage);
        }
    }

    max_joltage
}

fn solve(input_file: &str) -> u32 {
    /// Solve Day 3 Part 1: Maximum joltage from battery banks.
    let contents = fs::read_to_string(input_file)
        .expect("Failed to read input file");

    let banks: Vec<&str> = contents
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect();

    let mut total = 0;
    for bank in banks {
        let max_joltage = find_max_joltage(bank);
        total += max_joltage;
    }

    total
}

fn main() {
    let result = solve("input.txt");
    println!("Total output joltage: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        assert_eq!(find_max_joltage("987654321111111"), 98);
        assert_eq!(find_max_joltage("811111111111119"), 89);
        assert_eq!(find_max_joltage("234234234234278"), 78);
        assert_eq!(find_max_joltage("818181911112111"), 92);
    }

    #[test]
    fn test_total() {
        let test_cases = vec![
            "987654321111111",
            "811111111111119",
            "234234234234278",
            "818181911112111",
        ];
        let total: u32 = test_cases.iter().map(|&bank| find_max_joltage(bank)).sum();
        assert_eq!(total, 357);
    }
}
