use std::fs;

fn is_invalid_id(num: u64) -> bool {
    let s = num.to_string();
    let length = s.len();

    // Must have even length to be repeated twice
    if length % 2 != 0 {
        return false;
    }

    // Split in half and check if both halves are identical
    let mid = length / 2;
    let first_half = &s[..mid];
    let second_half = &s[mid..];

    first_half == second_half
}

fn solve(input_file: &str) -> u64 {
    let content = fs::read_to_string(input_file)
        .expect("Failed to read input file")
        .trim()
        .to_string();

    let mut total_sum: u64 = 0;

    // Parse ranges
    for range_str in content.split(',') {
        if !range_str.contains('-') {
            continue;
        }

        let parts: Vec<&str> = range_str.split('-').collect();
        if parts.len() != 2 {
            continue;
        }

        let start: u64 = parts[0].parse().expect("Failed to parse start");
        let end: u64 = parts[1].parse().expect("Failed to parse end");

        // Check all numbers in this range
        for num in start..=end {
            if is_invalid_id(num) {
                total_sum += num;
            }
        }
    }

    total_sum
}

fn main() {
    let result = solve("input.txt");
    println!("Sum of all invalid IDs: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_invalid_id() {
        assert_eq!(is_invalid_id(11), true);
        assert_eq!(is_invalid_id(22), true);
        assert_eq!(is_invalid_id(99), true);
        assert_eq!(is_invalid_id(1010), true);
        assert_eq!(is_invalid_id(123123), true);
        assert_eq!(is_invalid_id(6464), true);
        assert_eq!(is_invalid_id(55), true);
        assert_eq!(is_invalid_id(101), false);
        assert_eq!(is_invalid_id(1234), false);
    }
}
