def parse_input():
    return [list(map(int, line.split())) for line in open(0)]

def solve_p1(left, right):
    return sum(abs(l - r) for l, r in zip(left, right))

def solve_p2(left, right):
    right_occurrences = {x: right.count(x) for x in set(right)}
    return sum(l * right_occurrences.get(l, 0) for l in left)

lines = parse_input()
left, right = zip(*lines)
left, right = sorted(left), sorted(right)

print("Part 1:", solve_p1(left, right))
print("Part 2:", solve_p2(left, right))
