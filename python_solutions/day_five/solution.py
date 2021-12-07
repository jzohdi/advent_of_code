from typing import Callable
# part one: at how many points do at least 2 lines overalp?

def iter_coords(start_x, start_y, end_x, end_y):
    if start_x == end_x:
        for y in range(min(start_y, end_y), max(start_y, end_y) + 1):
            yield get_board_key(start_x, y)
    elif start_y == end_y:
        for x in range(min(start_x, end_x), max(start_x, end_x) + 1):
            yield get_board_key(x, start_y)
    else:
        step_x = 1 if end_x > start_x else -1
        step_y = 1 if end_y > start_y else -1
        y = -1
        for x in range(start_x, end_x + step_x, step_x):
            y += 1
            yield get_board_key(x, start_y + (y * step_y))

def part_one(file_path: str, should_use_points: Callable[[int, int, int, int], bool]) -> int:
    board = {}
    answer = 0
    # max_x, max_y = 0, 0
    with open(file_path) as file:
        for line in file.readlines():
            (start_x, start_y, end_x, end_y) = parse_line_to_coords(line)
            if not should_use_points(start_x, start_y, end_x, end_y):
                continue
            # max_x = max(max_x, max(start_x, end_x))
            # max_y = max(max_y, max(start_y, end_y))
            for key in iter_coords(start_x, start_y, end_x, end_y):
                curr_num = board.get(key, 0)
                next_num = curr_num + 1
                board[key] = next_num
                if next_num == 2:
                    answer += 1
    # print_board(board, max_x, max_y)
    return answer



def print_board(board, max_x, max_y):
    for y in range(max_y + 1):
        for x in range(max_x + 1):
            key = get_board_key(x, y)
            if key in board:
                print(board[key], end="")
            else:
                print(".", end="")
        print("")

def get_board_key(x: int, y: int) -> str:
    return str(x) + "-" + str(y)

def parse_line_to_coords(line: str) -> 'tuple[int, int, int, int]':
    coords = line.strip().split("->")
    start = coords[0].strip().split(",")
    end = coords[1].strip().split(",")
    return (int(start[0]), int(start[1]), int(end[0]), int(end[1]))

def is_straight_or_diagonal(s_x: int, s_y: int, e_x: int, e_y: int) -> bool:
    if s_x == e_x or s_y == e_y:
        return True

    if abs(s_x - e_x) == abs(s_y - e_y):
        return True
    return False

if __name__ == '__main__':
    file_path = './input.txt'
    print("Day 5, part one:", part_one(file_path, lambda s_x, s_y, e_x, e_y: s_x == e_x or s_y == e_y))
    print("Day 5, part two:", part_one(file_path, is_straight_or_diagonal))