def parse_input():
    with open(0) as f:
        return [line.strip() for line in f.readlines()]


def solve_p1(input):
    maxRed, maxGreen, maxBlue = 12, 13, 14
    games = get_games(input)
    result_sum = sum(range(1, len(games) + 1))

    for index, game in enumerate(games):
        for reveal in game:
            cubes = [cube.rstrip(',') for cube in reveal.split(' ') if cube]
            colors = cubes[1::2]
            values = [int(cubes[i]) for i in range(0, len(cubes), 2)]

            color_sums = {'red': 0, 'green': 0, 'blue': 0}
            for color, value in zip(colors, values):
                color_sums[color] += value

            if any(color_sums[color] > maxVal for color,
                   maxVal in zip(['red', 'green', 'blue'],
                                 [maxRed, maxGreen, maxBlue])):
                result_sum -= index + 1
                break

    return result_sum


def solve_p2(input):
    games = get_games(input)

    powerSum = 0

    for game in games:
        lowest_colors = {'red': 0, 'green': 0, 'blue': 0}

        for reveal in game:
            cubes = [cube.rstrip(',') for cube in reveal.split(' ') if cube]
            colors = cubes[1::2]
            values = [int(cubes[i]) for i in range(0, len(cubes), 2)]

            for color, value in zip(colors, values):
                if value > lowest_colors[color]:
                    lowest_colors[color] = value

        powerSum += lowest_colors['red'] * lowest_colors['green'] * lowest_colors['blue']

    return powerSum


def get_games(input):
    return [line.strip().split(': ', 1)[1].split(';') for line in input]


lines = parse_input()

print(solve_p1(lines))
print(solve_p2(lines))
