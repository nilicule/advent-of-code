def solve_safe_combination(filename):
    """
    Solve the safe combination puzzle (Part 1).

    The dial has positions 0-99 in a circle.
    Start at position 50.
    Process rotations (L=left/lower, R=right/higher).
    Count how many times the dial points at 0 after any rotation.
    """
    with open(filename, 'r') as f:
        rotations = f.read().strip().split('\n')

    position = 50
    zero_count = 0

    for rotation in rotations:
        direction = rotation[0]
        distance = int(rotation[1:])

        if direction == 'L':
            position = (position - distance) % 100
        else:  # direction == 'R'
            position = (position + distance) % 100

        if position == 0:
            zero_count += 1

    return zero_count


if __name__ == "__main__":
    # Solve the actual puzzle
    result = solve_safe_combination('input.txt')
    print(f"The actual password is: {result}")
