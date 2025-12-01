def solve_safe_combination_with_clicks(filename):
    """
    Solve the safe combination puzzle using method 0x434C49434B.

    Count every time the dial passes through 0 during a rotation,
    not just when it ends on 0.

    The dial has positions 0-99 in a circle.
    Start at position 50.
    Process rotations (L=left/lower, R=right/higher).
    Count how many times the dial points at 0 during any rotation.
    """
    with open(filename, 'r') as f:
        rotations = f.read().strip().split('\n')

    position = 50
    zero_count = 0

    for rotation in rotations:
        direction = rotation[0]
        distance = int(rotation[1:])

        # Simulate the rotation click by click
        current = position
        for _ in range(distance):
            if direction == 'R':
                current = (current + 1) % 100
            else:  # 'L'
                current = (current - 1) % 100

            if current == 0:
                zero_count += 1

        position = current

    return zero_count


if __name__ == "__main__":
    # Solve the actual puzzle
    result = solve_safe_combination_with_clicks('input.txt')
    print(f"The actual password (method 0x434C49434B) is: {result}")
