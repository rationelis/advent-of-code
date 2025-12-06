with open(0) as f:
    lines = [line.strip() for line in f.readlines()]

grid = [[int(val) for val in line] for line in lines]

zero_positions = [
    (i, j) for i, row in enumerate(grid) for j, val in enumerate(row) if val == 0
]

print(zero_positions)
