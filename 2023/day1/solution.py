import re

with open("day1/input.txt") as f:
    lines = f.readlines()
    s = 0
    for line in lines:
       digits = re.findall(r'\d', line)
       s += int(digits[0]) + int(digits[-1])
    print(s)
