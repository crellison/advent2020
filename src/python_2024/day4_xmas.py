from copy import deepcopy
from collections import defaultdict
from os.path import abspath, dirname
from typing import List

INPUT_FILE = abspath(dirname(abspath(__file__)) + "/../../input/2024/4-1.txt")


def get_input() -> List:
    contents = open(INPUT_FILE).read().split("\n")
    if contents[-1] == "":
        contents.pop()
    return contents


directions = [(i, j) for i in range(-1, 2) for j in range(-1, 2) if i != 0 or j != 0]


def part_one(input: List) -> int:
    word = "XMAS"

    def check_word(x, y, dx, dy):
        for i in range(1, len(word)):
            next_x = x + dx * i
            next_y = y + dy * i
            if 0 <= next_x < len(input) and 0 <= next_y < len(input[next_x]):
                if input[next_x][next_y] != word[i]:
                    return False
            else:
                return False
        return True

    word_count = 0
    for x in range(len(input)):
        for y in range(len(input[x])):
            if input[x][y] != word[0]:
                continue

            for dx, dy in directions:
                if check_word(x, y, dx, dy):
                    word_count += 1

    return word_count


def part_two(input: List) -> int:
    mas_count = 0

    def check_mas(x, y):
        if not (1 <= x < len(input) - 1 and 1 <= y < len(input[x]) - 1):
            return False  # need to not be on an edge

        right_diagonal = input[x - 1][y - 1] + input[x + 1][y + 1]
        left_diagonal = input[x - 1][y + 1] + input[x + 1][y - 1]

        if right_diagonal in ["MS", "SM"] and left_diagonal in ["MS", "SM"]:
            return True
        return False

    for x in range(len(input)):
        for y in range(len(input[x])):
            if input[x][y] != "A":
                continue
            if check_mas(x, y):
                mas_count += 1
    return mas_count


if __name__ == "__main__":
    input = get_input()
    print(f"Part 1: {part_one(input)}")
    print(f"Part 2: {part_two(input)}")
