from copy import deepcopy
from collections import defaultdict
from os.path import abspath, dirname
from typing import List

INPUT_FILE = abspath(dirname(abspath(__file__)) + "/../../input/2024/12-test.txt")

def get_input() -> List:
    contents = open(INPUT_FILE).read().split("\n")
    if contents[-1] == "":
        contents.pop()
    return contents

def part_one(input: List) -> int:
    return 0

def part_two(input: List) -> int:
    return 0

if __name__ == "__main__":
    input = get_input()
    print(f"Part 1: {part_one(input)}")
    print(f"Part 2: {part_two(input)}")
