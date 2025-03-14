def print_solution(array):
    print("".join(str(e) if e != -1 else "." for e in array))

with open(0) as f:
    line = f.readlines()[0].strip()

array, id, alloc_space = [], 0, False

for num in map(int, line):
    array.extend([-1] * num if alloc_space else [id] * num)
    id += not alloc_space
    alloc_space = not alloc_space

print_solution(array)

for (index, elem) in reversed(list(enumerate(array))):
    if elem != -1:
        # We swap with first 
        swap_index = 0
        for (seq_index, seq_elem) in enumerate(array):
            if seq_elem == -1:
                swap_index = seq_index
                break
        array[swap_index] = elem
        array[index] = -1

print_solution(array)

