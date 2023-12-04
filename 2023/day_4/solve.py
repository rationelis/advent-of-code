import re


def parse_input():
    with open(0) as f:
        return [line.strip() for line in f.readlines()]


def solve_p1():
    games = []
    for line in lines:
        parts = line.split(" | ")
        winning_numbers = [int(x)
                           for x in parts[0].split(": ")[1].split(" ") if x]
        our_numbers = [int(x) for x in parts[1].split(" ") if x]
        games.append((winning_numbers, our_numbers))

    sum = 0
    for game in games:
        winning_numbers, our_numbers = game
        point = 1
        total = 0
        for winning_number in winning_numbers:
            if winning_number in our_numbers:
                total = point
                point *= 2
        sum += total

    return sum


def solve_p2():
    games = []
    for line in lines:
        parts = line.split(" | ")
        winning_numbers = {int(x)
                           for x in parts[0].split(": ")[1].split(" ") if x}
        our_numbers = {int(x) for x in parts[1].split(" ") if x}
        games.append((winning_numbers, our_numbers))

    matchesPerGame = [len(winning_numbers.intersection(our_numbers))
                      for winning_numbers, our_numbers in games]

    occ = {}
    todo = list(range(len(games)))
    while todo:
        x = todo.pop(0)
        occ[x] = occ.get(x, 0) + 1
        toAdd = list(range(x + 1, min(x + matchesPerGame[x] + 1, len(games))))
        todo.extend(toAdd)

    total = sum(v for k, v in occ.items() if k >= 0)
    return total


lines = parse_input()

# print(solve_p1())
print(solve_p2())
