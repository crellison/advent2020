from copy import deepcopy
from collections import defaultdict
from os.path import abspath, dirname
from typing import List, Tuple
import heapq

INPUT_FILE = abspath(dirname(abspath(__file__)) + "/../../input/2024/16-1.txt")

directions = {
    "E": {
        "move": (0, 1),
        "turns": ["S", "N"],
    },
    "W": {
        "move": (0, -1),
        "turns": ["N", "S"],
    },
    "S": {
        "move": (1, 0),
        "turns": ["E", "W"],
    },
    "N": {
        "move": (-1, 0),
        "turns": ["W", "E"],
    },
}


def get_input() -> Tuple[List, Tuple, Tuple]:
    contents = open(INPUT_FILE).read().split("\n")
    if contents[-1] == "":
        contents.pop()
    start_index = min(
        (i, j)
        for i in range(len(contents))
        for j in range(len(contents[i]))
        if contents[i][j] == "S"
    )
    end_index = min(
        (i, j)
        for i in range(len(contents))
        for j in range(len(contents[i]))
        if contents[i][j] == "E"
    )
    return contents, start_index, end_index


def part_one(input: List, start_index: Tuple, end_index: Tuple) -> int:
    heap = []
    seen = set()
    heapq.heappush(heap, (0, start_index, "E"))
    while heap:
        cost, (x, y), direction = heapq.heappop(heap)
        if (x, y) == end_index:
            return cost
        if (x, y, direction) in seen:
            continue
        seen.add((x, y, direction))
        for turn in directions[direction]["turns"]:
            heapq.heappush(heap, (cost + 1000, (x, y), turn))

        next_x, next_y = (
            x + directions[direction]["move"][0],
            y + directions[direction]["move"][1],
        )
        if next_x in range(len(input)) and next_y in range(len(input[next_x])):
            if input[next_x][next_y] != "#":
                heapq.heappush(heap, (cost + 1, (next_x, next_y), direction))
    return 0


def part_two(input: List, start_index: Tuple, end_index: Tuple) -> int:
    heap = []
    position_costs = defaultdict(lambda: float("inf"))
    min_cost = float("inf")
    paths = set()
    heapq.heappush(heap, (0, start_index, "E", []))
    while heap:
        cost, (x, y), direction, path = heapq.heappop(heap)
        if cost > min_cost:
            continue
        if (x, y) == end_index:
            min_cost = min(min_cost, cost)
            paths.update(path)
            continue
        if position_costs[(x, y, direction)] < cost:
            continue
        position_costs[(x, y, direction)] = cost
        for turn in directions[direction]["turns"]:
            heapq.heappush(heap, (cost + 1000, (x, y), turn, path))

        next_x, next_y = (
            x + directions[direction]["move"][0],
            y + directions[direction]["move"][1],
        )
        if next_x in range(len(input)) and next_y in range(len(input[next_x])):
            if input[next_x][next_y] != "#":
                heapq.heappush(
                    heap, (cost + 1, (next_x, next_y), direction, [*path, (x, y)])
                )
    return len(paths) + 1


if __name__ == "__main__":
    input, start_index, end_index = get_input()
    print(f"Part 1: {part_one(input, start_index, end_index)}")
    print(f"Part 2: {part_two(input, start_index, end_index)}")
