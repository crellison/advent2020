from copy import deepcopy
from collections import defaultdict
from os.path import abspath, dirname
from typing import List
from functools import reduce
import re

INPUT_FILE = abspath(dirname(abspath(__file__)) + "/../../input/2024/14-1.txt")
# floor width and height
# floor_dimensions = (11, 7)  # for 14-test.txt
floor_dimensions = (101, 103)  # for 14-1.txt

"""
p=0,4 v=3,-3
p=6,3 v=-1,-3
"""


def get_input() -> List:
    contents = open(INPUT_FILE).read().split("\n")
    if contents[-1] == "":
        contents.pop()
    return [
        list(
            map(
                int,
                re.match(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)", content).groups(),
            )
        )
        for content in contents
    ]


def part_one(input: List) -> int:
    floor_x, floor_y = floor_dimensions
    quadrant_counts = [0, 0, 0, 0]
    for [x, y, dx, dy] in input:
        x_loc = (x + 100 * dx) % floor_x
        y_loc = (y + 100 * dy) % floor_y
        if x_loc == floor_x // 2 or y_loc == floor_y // 2:
            continue
        quadrant_index = 0
        if x_loc > floor_x // 2:
            quadrant_index += 1
        if y_loc > floor_y // 2:
            quadrant_index += 2
        quadrant_counts[quadrant_index] += 1

    return reduce(lambda x, y: x * y, quadrant_counts)


def part_two(input: List) -> int:
    floor_x, floor_y = floor_dimensions
    all_neighbors = [
        (1, 1),
        (1, 0),
        (1, -1),
        (0, 1),
        (0, -1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
    ]
    for i in range(10000):
        locations = defaultdict(lambda: defaultdict(int))
        will_print = False
        for [x, y, dx, dy] in input:
            x_loc = (x + i * dx) % floor_x
            y_loc = (y + i * dy) % floor_y
            locations[x_loc][y_loc] += 1
            if not will_print:
                will_print = all(
                    locations[(x_loc + dx) % floor_x][(y_loc + dy) % floor_y] != 0
                    for dx, dy in all_neighbors
                )

        if not will_print:
            continue

        map = ""
        for y in range(floor_y):
            for x in range(floor_x):
                map += "." if locations[x][y] == 0 else str(locations[x][y])
            map += "\n"

        print(f"\n ============= iteration {i} =============\n")
        print(map)

    return


if __name__ == "__main__":
    input = get_input()
    print(f"Part 1: {part_one(input)}")
    print(f"Part 2: {part_two(input)}")
