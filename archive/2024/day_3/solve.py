import re

def parse_input():
    with open(0) as f:
        return "\n".join(f.readlines())

line = parse_input()

pattern = r"mul\((\d+),(\d+)\)|(do\(\))|(don't\(\))"

p1 = p2 = 0
enabled = True
for a, b, do, dont in re.findall(pattern, line):
    if do or dont:
        enabled = bool(do)
    else:
        x = int(a) * int(b)
        p1 += x
        p2 += x * enabled

print("Part 1: %d" % p1)
print("Part 2: %d" % p2)
