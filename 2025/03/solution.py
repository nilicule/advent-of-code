def find_max_joltage(bank):
    """Find the maximum joltage possible from a bank of batteries.

    We need to pick exactly two batteries at positions i and j (where i < j)
    and concatenate them to form the largest possible number. We cannot
    rearrange the batteries, so we try all possible pairs.
    """
    max_joltage = 0
    n = len(bank)

    # Try all pairs (i, j) where i < j
    for i in range(n):
        for j in range(i + 1, n):
            # Form the number by concatenating digits at positions i and j
            joltage = int(bank[i] + bank[j])
            max_joltage = max(max_joltage, joltage)

    return max_joltage


def solve(input_file):
    """Solve Day 3 Part 1: Maximum joltage from battery banks."""
    with open(input_file) as f:
        banks = [line.strip() for line in f if line.strip()]

    total = 0
    for bank in banks:
        max_joltage = find_max_joltage(bank)
        total += max_joltage

    return total


if __name__ == "__main__":
    result = solve("input.txt")
    print(f"Total output joltage: {result}")
