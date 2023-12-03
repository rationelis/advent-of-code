special_characters = "!@#$%^&*()-+?_=,<>/"


def parse_input():
    with open(0) as f:
        return [line.strip() for line in f.readlines()]


def create_grid(lines):
    grid = []
    for line in lines:
        grid.append(list(line))
    return grid


def solve_p1(input):
    grid = create_grid(input)

    all_nums = []

    for x in range(len(grid)):
        for y in range(len(grid[x])):
            if grid[x][y].isdigit():
                all_nums.append((x, y))

    good_nums = []

    for x, y in all_nums:
        for dx, dy in [(-1, 0), (1, 0), (0, -1), (0, 1),
                       (-1, -1), (1, -1), (-1, 1), (1, 1)]:
            new_x, new_y = x + dx, y + dy

            if 0 <= new_y < len(grid) and 0 <= new_x < len(grid[new_y]) and grid[new_x][new_y] in special_characters:
                print(f"Found {grid[new_x][new_y]} next to {grid[x][y]} at X{new_x}, Y{new_y}")
                good_nums.append((x, y))

            if x > 0:
                break

    return
    print(good_nums)
    return
    # Go through all integers and look-ahead for following integers.
    # If there is a following integer, add it to to-be-integer.
    # If there is not, add the current integer to the list.
    # [(0,0),(0,1),(0,2)] -> take these three values and make them an integer.
    sum = 0
    numToBe = ""
    toSkip = []
    for x, y in good_nums:
        if (x, y) in toSkip:
            continue
        numToBe += grid[x][y]
        print(f"is (x, y+1) in good_nums? {x, y+1} in -> {(x+1, y) in good_nums}")
        return
        if x + 1 < len(grid) and (x + 1, y) in good_nums:
            numToBe += grid[x + 1][y]
            print(numToBe)
            toSkip.append((x + 1, y))
        # else:
            # sum += int(numToBe)
            # numToBe = ""
        return

    return sum


lines = parse_input()

print(solve_p1(lines))
