def print_solution(array):
    print("".join(str(e) if e != -1 else "." for e in array))

def calc_checksum(array):
    return sum(index * value for index, value in enumerate(array) if (value != -1))

with open(0) as f:
    line = f.readlines()[0].strip()

array, id, alloc_space = [], 0, False

for num in map(int, line):
    array.extend([-1] * num if alloc_space else [id] * num)
    id += not alloc_space
    alloc_space = not alloc_space

def solve_p1(array):
    left = 0
    for index in range(len(array) - 1, -1, -1):
        if left >= index:
            break
        if array[index] == -1:
            continue
        while left < len(array) and array[left] != -1:
            left += 1
        if left < index:
            array[left], array[index] = array[index], -1
    return calc_checksum(array)

def solve_p2(array):
    file_positions = {}
    i = 0
    while i < len(array):
        if array[i] != -1:
            file_id = array[i]
            start = i
            while i < len(array) and array[i] == file_id:
                i += 1
            file_positions[file_id] = (start, i - start)
        else:
            i += 1
    for file_id in sorted(file_positions.keys(), reverse=True):
        start, size = file_positions[file_id]
        free_start = None
        free_length = 0
        for i in range(len(array)):
            if array[i] == -1:
                if free_length == 0:
                    free_start = i
                free_length += 1
                if free_length == size:
                    break
            else:
                free_start = None
                free_length = 0
        if free_start is not None and free_start < start:
            for j in range(size):
                array[free_start + j] = file_id
                array[start + j] = -1
    return calc_checksum(array)

print(f"Part 1: {solve_p1(array[:])}")
print(f"Part 2: {solve_p2(array[:])}")
