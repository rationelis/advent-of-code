def parse_input():
    with open(0) as f:
        return [line.strip() for line in f.readlines()]
    

def get_ranges(lines):
    seeds = list(map(int, lines[0].split(': ')[1].split(' ')))
    # print(seeds)

    rows_of_ints = [list(map(int, line.split())) for line in lines[1:]]
    print(rows_of_ints)
    
    return None, None

def solve_p1(lines):
    seeds, ranges = get_ranges(lines)
    pass


lines = parse_input()

print(solve_p1(lines))