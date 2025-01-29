import re

def parse_input():
    with open(0) as f:
        return [line.strip() for line in f.readlines()]

def solve_p1(report):
    return all(1 <= abs(n1 - n2) <= 3 for n1, n2 in zip(report, report[1:])) and \
           (sorted(report) == report or sorted(report, reverse=True) == report)

def solve_p2(report):
    return any(solve_p1(report[:i] + report[i + 1:]) for i in range(len(report)))

lines = parse_input()

reports = [[int(num) for num in re.findall(r'\d+', line)] for line in lines]

p1 = sum(solve_p1(report) for report in reports)
print("Part 1: %d" % p1)

p2 = sum(solve_p2(report) for report in reports)
print("Part 2: %d" % p2)
