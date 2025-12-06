def parse_input():
    with open(0) as f:
        return [line.strip() for line in f.readlines()]


def get_ranges(lines):
    seeds = list(map(int, lines[0].split(': ')[1].split(' ')))
    lines = lines[1:]

    arr = []
    current_map_data = []

    for line in lines:
        if line.strip() and not line.endswith("map:"):
            current_map_data.append(list(map(int, line.split())))
        elif current_map_data:
            arr.append(current_map_data)
            current_map_data = []

    if current_map_data:
        arr.append(current_map_data)

    return seeds, arr


def solve_p1(lines):
    seeds, allRanges = get_ranges(lines)

    lowest = 2 ** 64 - 1
    for seed in seeds:
        for ranges in allRanges:
            for r in ranges:
                dest, src, length = r[0], r[1], r[2]
                if seed >= src and seed < src + length:
                    seed += dest - src
                    break
        if seed < lowest:
            lowest = seed

    return lowest


def solve_p2(lines):
    seeds, allRanges = get_ranges(lines)

    seedRanges = []
    for i in range(0, len(seeds), 2):
        seedRanges.append((seeds[i], seeds[i + 1]))

    lowest = 2 ** 64 - 1
    for seedRange in seedRanges:
        for i in range(seedRange[0], seedRange[0] + seedRange[1]):
            for ranges in allRanges:
                for r in ranges:
                    dest, src, length = r[0], r[1], r[2]
                    if i >= src and i < src + length:
                        i += dest - src
                        break
            if i < lowest:
                lowest = i

    return lowest


lines = parse_input()

print(solve_p1(lines))
print(solve_p2(lines))
