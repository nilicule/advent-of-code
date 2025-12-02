use std::fs;

fn is_invalid_id(num: u64) -> bool {
    let s = num.to_string();
    let length = s.len();

    // Try all possible pattern lengths from 1 to length//2
    // A pattern repeated at least twice means the pattern length is at most length//2
    for pattern_len in 1..=length / 2 {
        // Check if the string length is divisible by pattern length
        if length % pattern_len == 0 {
            let pattern = &s[..pattern_len];
            let repetitions = length / pattern_len;

            // Check if the pattern repeats at least twice
            if repetitions >= 2 {
                // Check if the entire string is made of this pattern repeated
                let repeated = pattern.repeat(repetitions);
                if repeated == s {
                    return true;
                }
            }
        }
    }

    false
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
        // From part 1
        assert_eq!(is_invalid_id(11), true);
        assert_eq!(is_invalid_id(22), true);
        assert_eq!(is_invalid_id(99), true);
        assert_eq!(is_invalid_id(1010), true);
        assert_eq!(is_invalid_id(123123), true);
        assert_eq!(is_invalid_id(6464), true);
        assert_eq!(is_invalid_id(55), true);
        assert_eq!(is_invalid_id(101), false);
        assert_eq!(is_invalid_id(1234), false);

        // New cases for part 2
        assert_eq!(is_invalid_id(111), true);
        assert_eq!(is_invalid_id(999), true);
        assert_eq!(is_invalid_id(12341234), true);
        assert_eq!(is_invalid_id(123123123), true);
        assert_eq!(is_invalid_id(1212121212), true);
        assert_eq!(is_invalid_id(1111111), true);
        assert_eq!(is_invalid_id(565656), true);
        assert_eq!(is_invalid_id(824824824), true);
        assert_eq!(is_invalid_id(2121212121), true);
    }
}
