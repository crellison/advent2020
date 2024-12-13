from copy import deepcopy
from collections import defaultdict
from os.path import abspath, dirname
from typing import List
import re

INPUT_FILE = abspath(dirname(abspath(__file__)) + "/../../input/2024/13-1.txt")


def get_input() -> List:
    contents = open(INPUT_FILE).read().split("\n\n")
    return [
        list(
            map(
                int,
                re.match(
                    r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X\=(\d+), Y\=(\d+)",
                    content,
                ).groups(),
            )
        )
        for content in contents
    ]


"""
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

94x + 22y = 8400
34x + 67y = 5400

ax + by = c
dx + ey = f

x = (c * e - b * f) / (a * e - b * d)
y = (a * f - c * d) / (a * e - b * d)
"""


def part_one(input: List) -> int:
    total_tokens = 0
    for [a, d, b, e, c, f] in input:
        x = (c * e - b * f) / (a * e - b * d)
        y = (a * f - c * d) / (a * e - b * d)
        if x.is_integer() and y.is_integer():
            total_tokens += x * 3 + y
    return total_tokens


def part_two(input: List) -> int:
    total_tokens = 0
    for [a, d, b, e, c, f] in input:
        c = c + 10000000000000
        f = f + 10000000000000
        x = (c * e - b * f) / (a * e - b * d)
        y = (a * f - c * d) / (a * e - b * d)
        if x.is_integer() and y.is_integer():
            total_tokens += x * 3 + y
    return total_tokens


if __name__ == "__main__":
    input = get_input()
    print(f"Part 1: {part_one(input)}")
    print(f"Part 2: {part_two(input)}")
