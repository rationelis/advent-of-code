def parse_input():
    with open(0) as f:
        return [line.strip() for line in f.readlines()]


def solve_p1():
    grid = []
    for line in lines:
        grid.append(list(line))

    special_characters = "!@#$%^&*()-+?_=,<>/"

    num_groups = []
    num_to_add = []
    for x in range(len(grid)):
        for y in range(len(grid[x])):
            if grid[x][y].isdigit():
                if num_to_add == []:
                    num_to_add.append((x, y))
                elif num_to_add[-1][0] == x and num_to_add[-1][1] == y - 1:
                    num_to_add.append((x, y))
            elif num_to_add != []:
                num_groups.append(num_to_add)
                num_to_add = []

    special_characters = "!@#$%^&*()-+?_=,<>/"
    sum = 0
    found_valid_value = False

    for num_group in num_groups:
        for x, y in num_group:
            for dx, dy in [(-1, 0), (1, 0), (0, -1), (0, 1),
                           (-1, -1), (1, -1), (-1, 1), (1, 1)]:
                new_x, new_y = x + dx, y + dy

                if 0 <= new_y < len(grid) and 0 <= new_x < len(grid[new_y]) and grid[new_x][new_y] in special_characters:
                    sum += int("".join([grid[x][y] for x, y in num_group]))
                    found_valid_value = True
                    break

            if found_valid_value:
                found_valid_value = False
                break

    return sum


def solve_p2():
    grid = []
    for line in lines:
        grid.append(list(line))

    num_groups = []
    gears = []
    num_to_add = []
    for x in range(len(grid)):
        for y in range(len(grid[x])):
            if grid[x][y].isdigit():
                if num_to_add == []:
                    num_to_add.append((x, y))
                elif num_to_add[-1][0] == x and num_to_add[-1][1] == y - 1:
                    num_to_add.append((x, y))
            elif num_to_add != []:
                num_groups.append(num_to_add)
                num_to_add = []
            if grid[x][y] == "*":
                gears.append((x, y))

    sum = 0
    for gear in gears:
        num_adjacent = 0
        potential_number = []
        for group in num_groups:
            for x, y in group:
                if abs(x - gear[0]) <= 1 and abs(y - gear[1]) <= 1:
                    num_adjacent += 1
                    potential_number.append(group)
                    break

        if num_adjacent == 2:
            val_1 = int("".join([grid[x][y] for x, y in potential_number[0]]))
            val_2 = int("".join([grid[x][y] for x, y in potential_number[1]]))
            sum += val_1 * val_2

    return sum


lines = parse_input()

print(solve_p1())
print(solve_p2())
