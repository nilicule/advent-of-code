def is_invalid_id(num):
    """
    Check if a number is invalid (made of a sequence repeated at least twice).
    E.g., 55 (5 repeated 2x), 111 (1 repeated 3x), 12341234 (1234 repeated 2x),
    123123123 (123 repeated 3x), 1212121212 (12 repeated 5x)
    """
    s = str(num)
    length = len(s)

    # Try all possible pattern lengths from 1 to length//2
    # A pattern repeated at least twice means the pattern length is at most length//2
    for pattern_len in range(1, length // 2 + 1):
        # Check if the string length is divisible by pattern length
        if length % pattern_len == 0:
            pattern = s[:pattern_len]
            repetitions = length // pattern_len

            # Check if the pattern repeats at least twice
            if repetitions >= 2:
                # Check if the entire string is made of this pattern repeated
                if pattern * repetitions == s:
                    return True

    return False


def solve(input_file):
    with open(input_file, 'r') as f:
        content = f.read().strip()

    # Parse ranges
    ranges = content.split(',')

    total_sum = 0

    for range_str in ranges:
        if '-' not in range_str:
            continue

        parts = range_str.split('-')
        start = int(parts[0])
        end = int(parts[1])

        # Check all numbers in this range
        for num in range(start, end + 1):
            if is_invalid_id(num):
                total_sum += num

    return total_sum


if __name__ == "__main__":
    result = solve("input.txt")
    print(f"Sum of all invalid IDs: {result}")
