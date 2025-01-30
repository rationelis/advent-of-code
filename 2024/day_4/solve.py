def parse_input():
    with open(0) as f:
        return [line.strip() for line in f.readlines()]

def solve_p1(lines, patterns):
    count = 0
    for i in range(0, len(lines)):
        for j in range(0, len(lines[i])):
            if lines[i][j] == "X":
                for k in range(len(patterns)):
                    remaining = []
                    temp_i = i
                    temp_j = j
                    dx, dy = patterns[k]
                    for _ in range(3):
                        new_x = temp_i + dx
                        new_y = temp_j + dy
                        if not (0 <= new_x < len(lines[i]) and 0 <= new_y < len(lines)):
                            break
                        remaining.append(lines[new_x][new_y])
                        temp_i += dx
                        temp_j += dy
                    if ''.join(remaining) == "MAS":
                        count += 1
    return count

def solve_p2(lines, patterns):
    count = 0
    for i in range(len(lines)):
        for j in range(len(lines[i])):
            if lines[i][j] == "A":
                remaining = []
                for k in range(0, len(patterns), 2):  
                    (dx_1, dy_1), (dx_2, dy_2) = patterns[k], patterns[k + 1]
                    new_x_1, new_y_1 = i + dx_1, j + dy_1
                    new_x_2, new_y_2 = i + dx_2, j + dy_2
                    if 0 <= new_x_1 < len(lines) and 0 <= new_y_1 < len(lines[0]) and \
                       0 <= new_x_2 < len(lines) and 0 <= new_y_2 < len(lines[0]):
                        remaining.append((lines[new_x_1][new_y_1], lines[new_x_2][new_y_2]))
                ms_count = sum(1 for pair in remaining if set(pair) == {'M', 'S'})
                if ms_count == 2:
                    count += 1
    return count

lines = parse_input()

p1_pattern = [
    (-1, 0),  (1, 0),   # Left, Right
    (0, -1),  (0, 1),   # Up, Down
    (-1, -1), (1, -1),  # Top-left, Top-right
    (-1, 1),  (1, 1)    # Bottom-left, Bottom-right
]

p2_pattern = [
    (-1, -1), (1, 1),   # Top-left & Bottom-right
    (-1, 1), (1, -1)    # Top-right & Bottom-left
]

print("Part 1: %d" % solve_p1(lines, p1_pattern))
print("Part 2: %d" % solve_p2(lines, p2_pattern))
