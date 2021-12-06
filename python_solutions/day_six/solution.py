
def init_fish(num: int):
    return { "rate": 7, "curr": num }

def step_fish(fish):
    curr = fish.get("curr")
    if curr == 0:
        curr_fish_rate = fish.get("rate")
        fish["curr"] = curr_fish_rate - 1
        return { "rate": 7, "curr": curr_fish_rate + 1}
    else:
        fish["curr"] = curr - 1
        return None


def part_one(file_path, num_days: int) -> int:
    fish = []

    with open(file_path) as file:
        line = file.readline()
        fish = list(map(lambda x: init_fish(int(x)), line.strip().split(",")))
    
    for day_num in range(num_days):
        curr_fish_len = len(fish)
        for fish_num in range(curr_fish_len):
            new_fish = step_fish(fish[fish_num])
            if new_fish is not None:
                fish.append(new_fish)
    return len(fish)

def part_two(file_path: str, num_days: int) -> int:
    fish = [0] * 9

    with open(file_path) as file:
        line = file.readline()
        start_state = list(map(lambda x: int(x), line.strip().split(",")))
        for state in start_state:
            fish[state] += 1
    
    for day_num in range(num_days):
        num_will_spawn = fish[0]
        for i in range(1, 9):
            fish[i - 1] = fish[i]
        fish[8] = num_will_spawn
        fish[6] += num_will_spawn
    return sum(fish)

def print_fish(fish):
    return str(list(map(lambda f: f.get("curr"), fish)))

if __name__ == '__main__':
    file_path = "./input.txt"
    days = 80
    print("Day 6, part one:", part_one(file_path, days))
    print("Day 6, part two:", part_two(file_path, 256))