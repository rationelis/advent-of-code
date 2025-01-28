import re

def parse_input():
    with open(0) as f:
        return [line.strip() for line in f.readlines()]

def is_sorted(lst):
    return all(lst[i] <= lst[i + 1] for i in range(len(lst) - 1)) or \
           all(lst[i] >= lst[i + 1] for i in range(len(lst) - 1))

def solve_p1(reports):
    reports = [report for report in reports if is_sorted(report)]
    total = 0
    for report in reports:
        valid = True
        for i in range(0, len(report) - 1):
            distance = abs(report[i] - report[i + 1])
            if distance < 1 or distance > 3:
                valid = False
                break
        if valid:
            total += 1
    return total

def solve_p2(reports):
    reports = [report for report in reports if is_sorted(report)]
    total = 0
    for report in reports:
        valid = True
        toleration_used = False
        for i in range(0, len(report) - 1):
            distance = abs(report[i] - report[i + 1])
            if distance < 1 or distance > 3:
                if not toleration_used:
                    toleration_used = True
                else:
                    valid = False
                    break
        if valid:
            total += 1
    return total

lines = parse_input()

reports = [[int(num) for num in re.findall(r'\d+', line)] for line in lines]

p1 = solve_p1(reports)
print("Part 1: %d" % p1)

p2 = solve_p2(reports)
print("Part 2: %d" % p2)

