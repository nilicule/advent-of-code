def is_invalid_id(num):
    """
    Check if a number is invalid (made of a sequence repeated twice).
    E.g., 55 (5 repeated), 6464 (64 repeated), 123123 (123 repeated)
    """
    s = str(num)
    length = len(s)

    # Must have even length to be repeated twice
    if length % 2 != 0:
        return False

    # Split in half and check if both halves are identical
    mid = length // 2
    first_half = s[:mid]
    second_half = s[mid:]

    return first_half == second_half


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
