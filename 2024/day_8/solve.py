from collections import defaultdict
from itertools import combinations

with open(0) as f:
    lines = [line.strip() for line in f.readlines()]

items = defaultdict(list)
{items[col].append((x, y)) for x, row in enumerate(lines) for y, col in enumerate(row) if col != "."}

all_edges = [list(combinations(items[dic], 2)) for dic in items.keys()]

def create_antinodes(edges, endless=False):
    antinodes = set()
    if endless:
        antinodes.update(val for vals in items.values() for val in vals)
    for edges in all_edges:
        for edge in edges:
            (x1, y1), (x2, y2) = edge
            dx = abs(x1 - x2)
            dy = abs(y1 - y2)
            while True:
                scope_violations = 0
                if x1 < x2 and y1 < y2: # Top-left
                    new_x1, new_y1 = x1 - dx, y1 - dy
                    new_x2, new_y2 = x2 + dx, y2 + dy
                elif x1 < x2 and y1 > y2: # Bottom-right
                    new_x1, new_y1 = x1 - dx, y1 + dy
                    new_x2, new_y2 = x2 + dx, y2 - dy
                elif x1 > x2 and y1 < y2: # Bottom-left
                    new_x1, new_y1 = x1 + dx, y1 - dy
                    new_x2, new_y2 = x2 - dx, y2 + dy
                elif x1 > x2 and y1 > y2: # Top-right
                    new_x1, new_y1 = x1 + dx, y1 + dy
                    new_x2, new_y2 = x2 - dx, y2 - dy
                def out_of_scope(x, y):
                    return x < 0 or x >= len(lines) or y < 0 or y >= len(lines[0])
                if not out_of_scope(new_x1, new_y1):
                    point1 = (new_x1, new_y1)
                    antinodes.add(point1)
                else:
                    scope_violations += 1
                if not out_of_scope(new_x2, new_y2):
                    point2 = (new_x2, new_y2)
                    antinodes.add(point2)
                else:
                    scope_violations += 1
                x1, y1 = new_x1, new_y1
                x2, y2 = new_x2, new_y2 
                if not endless or scope_violations == 2:
                    break
    return antinodes

def print_field(lines, antinodes):
    field = [list(row) for row in lines]
    for x, y in antinodes:
        if 0 <= x < len(field) and 0 <= y < len(field[0]):
            field[x][y] = '#'
    for row in field:
        print("".join(row)) 

p1_answer = create_antinodes(all_edges)
print(f"Part 1: {len(p1_answer)}")

p2_answer = create_antinodes(all_edges, True)
print(f"Part 2: {len(p2_answer)}")
