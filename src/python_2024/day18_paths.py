from copy import deepcopy
from collections import defaultdict
from os.path import abspath, dirname
from typing import List
import heapq

INPUT_FILE = abspath(dirname(abspath(__file__)) + "/../../input/2024/18-1.txt")


def get_input() -> List:
    contents = [
        tuple(map(int, line.split(",")))
        for line in open(INPUT_FILE).read().split("\n")
        if line != ""
    ]
    return contents


coord_range = range(71)


def find_path_to_exit(map: defaultdict) -> int:
    next_item = []
    heapq.heappush(next_item, (0, (0, 0)))
    seen = set()

    while len(next_item) > 0:
        cost, (x, y) = heapq.heappop(next_item)
        if (x, y) in seen:
            continue
        if (x, y) == (max(coord_range), max(coord_range)):
            return cost
        seen.add((x, y))
        for dx, dy in [(0, 1), (0, -1), (1, 0), (-1, 0)]:
            if (
                x + dx in coord_range
                and y + dy in coord_range
                and not map[(x + dx, y + dy)]
            ):
                heapq.heappush(next_item, (cost + 1, (x + dx, y + dy)))
    return -1


def part_one(input: List) -> int:
    map = defaultdict(bool)
    for x, y in input[:1024]:
        map[x, y] = True

    return find_path_to_exit(map)


def part_two(input: List) -> int:
    left = 0
    right = len(input)
    while left < right:
        mid = (left + right) // 2
        map = defaultdict(bool)
        for x, y in input[:mid]:
            map[x, y] = True
        if find_path_to_exit(map) == -1:
            right = mid
        else:
            left = mid + 1
    return input[left - 1]


if __name__ == "__main__":
    input = get_input()
    print(f"Part 1: {part_one(input)}")
    print(f"Part 2: {part_two(input)}")
