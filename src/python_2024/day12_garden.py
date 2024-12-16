from copy import deepcopy
from collections import defaultdict
from os.path import abspath, dirname
from typing import List
import queue

INPUT_FILE = abspath(dirname(abspath(__file__)) + "/../../input/2024/12-1.txt")

direct_neighbors = [(1, 0), (-1, 0), (0, 1), (0, -1)]

"""
AAAA
BBCD
BBCC
EEEC
"""


def get_input() -> List:
    contents = open(INPUT_FILE).read().split("\n")
    if contents[-1] == "":
        contents.pop()
    return contents


def get_region_for_coords(input: List, i: int, j: int) -> set:
    current_char = input[i][j]
    next_members = queue.Queue()
    next_members.put((i, j))
    region = set()
    checked = set()

    while not next_members.empty():
        x, y = next_members.get()
        region.add((x, y))
        for dx, dy in direct_neighbors:
            if (x + dx, y + dy) in region:
                continue
            if (x + dx, y + dy) in checked:
                continue
            checked.add((x + dx, y + dy))
            if (
                x + dx in range(len(input))
                and y + dy in range(len(input[x + dx]))
                and input[x + dx][y + dy] == current_char
            ):
                next_members.put((x + dx, y + dy))

    return region


def get_region_perimeter(region: set) -> int:
    perimeter = 0
    for x, y in region:
        for dx, dy in direct_neighbors:
            if (x + dx, y + dy) not in region:
                perimeter += 1
    return perimeter


def part_one(input: List) -> int:
    seen = set()
    total_price = 0
    for i, line in enumerate(input):
        for j, char in enumerate(line):
            if (i, j) in seen:
                # print(f"skipping {(i, j)}")
                continue
            region = get_region_for_coords(input, i, j)
            seen.update(region)
            # print(
            #     f"region {char} with {(i, j)} has perimeter {get_region_perimeter(region)} and area {len(region)}"
            # )
            total_price += get_region_perimeter(region) * len(region)

    return total_price


def get_region_side_count(region: set) -> int:
    """
    Calculates the number of sides of a given region.
    """
    side_count = 0
    edge_locations = {}
    for x, y in region:
        for dx, dy in direct_neighbors:
            if (x + dx, y + dy) not in region:
                if (x, y) not in edge_locations:
                    edge_locations[(x, y)] = {}
                edge_locations[(x, y)][(dx, dy)] = False

    for x, y in edge_locations:
        # print(f"checking sides for {(x,y)}")
        for dx, dy in edge_locations[(x, y)]:
            # print(f"checking side {(dx,dy)}")
            if edge_locations[(x, y)][(dx, dy)]:
                continue  # we have already checked this edge

            next = (x + dy, y + dx)
            while next in edge_locations:
                if (dx, dy) in edge_locations[next]:
                    # print(f"adding next {next} to side")
                    edge_locations[next][(dx, dy)] = True
                else:
                    break
                next = (next[0] + dy, next[1] + dx)

            next = (x - dy, y - dx)
            while next in edge_locations:
                if (dx, dy) in edge_locations[next]:
                    # print(f"adding next {next} to side")
                    edge_locations[next][(dx, dy)] = True
                else:
                    break
                next = (next[0] - dy, next[1] - dx)

            side_count += 1

    return side_count


def part_two(input: List) -> int:
    seen = set()
    total_price = 0
    for i, line in enumerate(input):
        for j, char in enumerate(line):
            if (i, j) in seen:
                # print(f"skipping {(i, j)}")
                continue
            region = get_region_for_coords(input, i, j)
            seen.update(region)
            # print(
            #     f"region {char} with {(i, j)} has side count {get_region_side_count(region)} and area {len(region)}"
            # )
            total_price += get_region_side_count(region) * len(region)

    return total_price


if __name__ == "__main__":
    input = get_input()
    print(f"Part 1: {part_one(input)}")
    print(f"Part 2: {part_two(input)}")
