def find_max_joltage_part2(bank, num_batteries=12):
    """Find the maximum joltage by picking exactly num_batteries batteries.

    Strategy: Use a greedy approach. At each step, pick the largest digit
    from the remaining candidates that still leaves enough digits to pick
    the required total.

    For example, if we need to pick 12 digits from a 15-digit bank:
    - We can skip at most 3 digits
    - At position i, we look ahead and find the largest digit in the window
      that ensures we can still pick enough remaining digits
    """
    n = len(bank)
    to_skip = n - num_batteries

    if to_skip < 0:
        # Not enough batteries in the bank
        return 0

    result = []
    start = 0  # Current position to search from

    for picked in range(num_batteries):
        # How many more batteries do we need after this one?
        remaining_needed = num_batteries - picked - 1

        # We can search up to position where we still have enough batteries left
        # to pick the remaining ones
        search_end = n - remaining_needed

        # Find the maximum digit in the valid search range
        max_digit = '0'
        max_pos = start

        for i in range(start, search_end):
            if bank[i] > max_digit:
                max_digit = bank[i]
                max_pos = i

        result.append(max_digit)
        start = max_pos + 1  # Next search starts after this position

    return int(''.join(result))


def solve_part2(input_file):
    """Solve Day 3 Part 2: Maximum joltage from battery banks (12 batteries each)."""
    with open(input_file) as f:
        banks = [line.strip() for line in f if line.strip()]

    total = 0
    for bank in banks:
        max_joltage = find_max_joltage_part2(bank)
        total += max_joltage

    return total


if __name__ == "__main__":
    result = solve_part2("input.txt")
    print(f"Total output joltage: {result}")

