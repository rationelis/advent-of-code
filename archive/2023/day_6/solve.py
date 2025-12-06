def parse_input():
    with open(0) as f:
        return [line.strip() for line in f.readlines()]
    

def get_races(lines):
    times = list(map(int, filter(None, lines[0].split('      ')[1].split(' '))))
    distances = list(map(int, filter(None, lines[1].split('  ')[1:])))
    return times, distances


def solve_p1(lines):
    times, distance = get_races(lines)
    races = list(zip(times, distance))

    product = 1
    for time, distance in races:
        ways_to_win = 0
        start_time = 1
        while start_time < time:
            new_distance = start_time * (time - start_time)
            if new_distance > distance:
                ways_to_win += 1
            start_time += 1
        product *= ways_to_win

    return product


def solve_p2(lines):
    times, distances = get_races(lines)

    time = int(''.join(map(str, times)))
    distance = int(''.join(map(str, distances)))

    races = [(time, distance)]

    product = 1
    for time, distance in races:
        ways_to_win = 0
        start_time = 1
        while start_time < time:
            new_distance = start_time * (time - start_time)
            if new_distance > distance:
                ways_to_win += 1
            start_time += 1
        product *= ways_to_win

    return product

lines = parse_input()

print(solve_p1(lines))
print(solve_p2(lines))
