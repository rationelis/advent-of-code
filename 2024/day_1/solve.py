def parse_input():
    with open(0) as f:
        return [line.strip() for line in f.readlines()]
   
def solve_p1(left, right):
    total_distance = 0
    for i in range(0, len(left)):
        total_distance += abs(left[i] - right[i])
    return total_distance

def solve_p2(left, right):
    right_occurrences = {}
    for entry in right:
        if entry not in right_occurrences:
            right_occurrences[entry] = 1
        else:
            right_occurrences[entry] += 1 
    score = 0
    for entry in left:
        if entry in right_occurrences:
            score += entry * right_occurrences[entry]
    return score

lines = parse_input()

left, right = [], []

for line in lines:
    entries = [int(s) for s in line.split()]
    left.append(entries[0])
    right.append(entries[1])

left.sort()
right.sort()

p1 = solve_p1(left, right)
print("Part 1: %d" % p1) 

p2 = solve_p2(left, right)
print("Part 2: %d" % p2) 
