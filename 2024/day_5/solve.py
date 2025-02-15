from collections import defaultdict

with open(0) as f:
    part1, part2 = f.read().strip().split("\n\n", 1)

rules = defaultdict(list)

for line in part1.split("\n"):
    left, right = map(int, line.split("|"))
    rules[left].append(right)

def insertion_sort(numbers):
    ordered = [numbers[0]]
    for num in numbers[1:]:
        inserted = False
        for i, existing in enumerate(ordered):
            if existing in rules and num in rules[existing]:
                ordered.insert(i, num)
                inserted = True
                break
        if not inserted:
            ordered.append(num)
    return ordered

middle_values_p1 = []
middle_values_p2 = []

for line in part2.split("\n"):
    nums = list(map(int, line.split(",")))
    for i, num in enumerate(nums):
        left = nums[:i]
        for l in left:
            if num in rules and l in rules[num]:
                new_sorted = insertion_sort(nums)
                middle_values_p2.append(new_sorted[len(new_sorted) // 2])
                break
        else:
            continue 
        break 
    else:
        middle_values_p1.append(nums[len(nums) // 2])


print(f"Part 1: {sum(middle_values_p1)}")
print(f"Part 2: {sum(middle_values_p2)}")
