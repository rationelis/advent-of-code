with open(0) as f:
    lines = [line.strip() for line in f.readlines()]

directions = [
    (-1, 0),  # Up
    (0, 1),   # Right
    (1, 0),   # Down
    (0, -1)   # Left
]

def solve_p1(x, y, current_direction):
    steps = 1
    visited = set()
    while True:
        dx, dy = directions[current_direction]
        new_x, new_y = x + dx, y + dy
        if new_x < 0 or new_x >= len(lines) or new_y < 0 or new_y >= len(lines[0]):
            break
        if lines[new_x][new_y] == "#":
            current_direction = (current_direction + 1) % len(directions)
            continue
        if (x, y) not in visited:
            visited.add((x, y))
            steps += 1
        x, y = new_x, new_y
    return steps

def solve_p2(x, y, current_direction):
    obstructions = 0
    for opt_x, row in enumerate(lines):
        for opt_y, col in enumerate(lines[0]):
            visited = set()
            current_x, current_y = x, y
            current_direction_temp = current_direction
            while True:
                dx, dy = directions[current_direction_temp]
                new_x, new_y = current_x + dx, current_y + dy
                if new_x < 0 or new_x >= len(lines) or new_y < 0 or new_y >= len(lines[0]):
                    break
                if lines[new_x][new_y] == "#" or (opt_x == new_x and opt_y == new_y):
                    current_direction_temp = (current_direction_temp + 1) % len(directions)
                    continue
                if (current_direction_temp, (current_x, current_y)) not in visited:
                    visited.add((current_direction_temp, (current_x, current_y)))
                else: 
                    obstructions += 1
                    break
                current_x, current_y = new_x, new_y
    return obstructions

x, y = next((x, y) for x, row in enumerate(lines) for y, col in enumerate(row) if col == "^")

print("Part 1: %s" % solve_p1(x, y, 0))
print("Part 2: %s" % solve_p2(x, y, 0))
