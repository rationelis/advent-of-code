from itertools import product

with open(0) as f:
    lines = [line.strip() for line in f.readlines()]

equations = []
for line in lines:
    parts = line.split(":")
    nums = list(parts[1].split())
    equations.append((int(parts[0]), nums))

def left_to_right_eval(nums, ops):
    result = int(nums[0])
    for i in range(len(ops)):
        if ops[i] == '+':
            result += int(nums[i + 1])
        elif ops[i] == '||':
            result = int(str(result) + nums[i + 1])
        else:  # '*'
            result *= int(nums[i + 1])
    return result

def solve(equations, ops):
    answer = 0
    for res, nums in equations:
        for op_seq in product(ops, repeat=len(nums)-1): 
            if left_to_right_eval(nums, op_seq) == res:
                answer += res
                break 
    return answer

p1_ops = ['+', '*']
p2_ops = ['+', '*', '||']

print(f"Part 1: {solve(equations, p1_ops)}")
print(f"Part 2: {solve(equations, p2_ops)}")
