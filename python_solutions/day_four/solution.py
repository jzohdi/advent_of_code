import re

"""
for each col and row, make a list of these:
[row1,
row2,
row3,
row4,
col1,
col2,
col3,
col4]

if any of these are empty,  then it is winning,
to take in a number is to loop over these lists and remove the number.
could make more efficient to have a dict { num: [row, col] }
"""
class Board:
    def __init__(self):
        self.board: 'list[list[str]]' = []
        self.is_winning: bool = False
        self.winning_num: int = None
    
    def parse_line(self, line: str) -> None:
        parsed = re.sub(' +', ' ', line).strip().split(" ")
        self.board.append(parsed)

    def done_parse(self) -> None:
        cols = [[row[i] for row in self.board] for i in range(len(self.board))]

        self.board += cols

    def mark(self, num: int) -> None:
        for row_col in self.board:
            if num in row_col:
                row_col.remove(num)
                if len(row_col) == 0:
                    self.is_winning = True
                    self.winning_num = num

    def has_won(self) -> bool:
        return self.is_winning

    def score(self) -> int:
        total = sum([int(num) 
                    for row in self.board[:5]
                    for num in row])
        print(total)
        return total * int(self.winning_num)

    def __repr__(self) -> str:
        return "Board:"+ "\n".join([" ".join(row) for row in self.board[:5]]) + "\n"

def load_boards() -> 'tuple[str,list[Board]]':
    start_line: str = None

    boards: 'list[Board]' = []

    with open("./input.txt") as file:
        curr_board = None
        for line in file.readlines():
            if start_line is None:
                start_line = line
            
            elif len(line) < 2:
                if curr_board is not None:
                    curr_board.done_parse() 
                curr_board = Board()
                boards.append(curr_board)
            else:
                curr_board.parse_line(line)

    boards[-1].done_parse()
    start_line = start_line.strip().split(",")
    return (start_line, boards)    

def part_one() -> None:
    (start_line, boards) = load_boards()

    for num in start_line:
        for board in boards:
            board.mark(num)
            if board.has_won():
                print(board, board.score())
                exit()

def part_two() -> None:
    (start_line, boards) = load_boards()

    how_many_won = 0
    for num in start_line:
        for board in boards:
            if not board.has_won():
                board.mark(num)
                if board.has_won():
                    how_many_won += 1
                if how_many_won == len(boards):
                    print(board.score())    

part_two()