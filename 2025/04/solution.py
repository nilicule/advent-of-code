def count_accessible_rolls(grid):
    """
    Count paper rolls (@) that have fewer than 4 adjacent paper rolls.
    Adjacent means all 8 surrounding positions (including diagonals).
    """
    rows = len(grid)
    cols = len(grid[0]) if rows > 0 else 0

    # All 8 directions: up, down, left, right, and 4 diagonals
    directions = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),           (0, 1),
        (1, -1),  (1, 0),  (1, 1)
    ]

    accessible_count = 0

    for i in range(rows):
        for j in range(cols):
            # Only check positions with paper rolls
            if grid[i][j] == '@':
                # Count adjacent paper rolls
                adjacent_rolls = 0
                for di, dj in directions:
                    ni, nj = i + di, j + dj
                    # Check if in bounds and is a paper roll
                    if 0 <= ni < rows and 0 <= nj < cols and grid[ni][nj] == '@':
                        adjacent_rolls += 1

                # Accessible if fewer than 4 adjacent rolls
                if adjacent_rolls < 4:
                    accessible_count += 1

    return accessible_count


def find_accessible_positions(grid):
    """
    Find all positions with paper rolls that have fewer than 4 adjacent paper rolls.
    Returns a set of (row, col) tuples.
    """
    rows = len(grid)
    cols = len(grid[0]) if rows > 0 else 0

    directions = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),           (0, 1),
        (1, -1),  (1, 0),  (1, 1)
    ]

    accessible = set()

    for i in range(rows):
        for j in range(cols):
            if grid[i][j] == '@':
                adjacent_rolls = 0
                for di, dj in directions:
                    ni, nj = i + di, j + dj
                    if 0 <= ni < rows and 0 <= nj < cols and grid[ni][nj] == '@':
                        adjacent_rolls += 1

                if adjacent_rolls < 4:
                    accessible.add((i, j))

    return accessible


def remove_accessible_rolls(grid):
    """
    Iteratively remove accessible paper rolls until no more can be removed.
    Returns the total number of rolls removed.
    """
    # Convert to mutable list of lists
    grid = [list(row) for row in grid]
    total_removed = 0

    while True:
        # Find all accessible positions
        accessible = find_accessible_positions(grid)

        if not accessible:
            break

        # Remove all accessible rolls
        for i, j in accessible:
            grid[i][j] = '.'

        total_removed += len(accessible)

    return total_removed


def main():
    with open('input.txt', 'r') as f:
        grid = [line.rstrip('\n') for line in f]

    result_part1 = count_accessible_rolls(grid)
    print(f"Part 1: {result_part1}")

    result_part2 = remove_accessible_rolls(grid)
    print(f"Part 2: {result_part2}")


if __name__ == '__main__':
    main()
