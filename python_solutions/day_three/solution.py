oxygen = []
co2 = []

with open("./input.txt") as file:
    for line in file.readlines():
        line = line.strip().replace("\n", "")
        oxygen.append(line)
        co2.append(line)

current_idx = 0
while len(oxygen) > 1:
    counter = {
        "0": 0,
        "1": 0
    }
    for line in oxygen:
        counter[line[current_idx]] += 1

    greatest = "1"
    if counter.get("0") > counter.get("1"):
        greatest = "0"

    oxygen = list(filter(lambda line: line[current_idx] == greatest, oxygen))
    current_idx += 1

current_idx = 0
while len(co2) > 1:
    counter = {
        "0": 0,
        "1": 0
    }
    for line in co2:
        counter[line[current_idx]] += 1

    least = "0"
    if counter.get("1") < counter.get("0"):
        least = "1"

    co2 = list(filter(lambda line: line[current_idx] == least, co2))
    current_idx += 1

print(int(oxygen[0], 2)* int(co2[0], 2))