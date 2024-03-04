file = open("input.txt")
lines = file.readlines()

highest = 0
elf = 0
elves = []

for line in lines:
    if line == "\n":
        elves.append(elf)
        elf = 0
        continue

    value = int(line.strip())

    elf += value

    if elf > highest:
        highest = elf


elves.sort(reverse=True)

print(elves[0] + elves[1] + elves[2])