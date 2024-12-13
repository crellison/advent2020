from copy import deepcopy
from collections import defaultdict
from os.path import abspath, dirname
from typing import List, Tuple
import queue

INPUT_FILE = abspath(dirname(abspath(__file__)) + "/../../input/2024/10-1.txt")


def get_input() -> List:
    contents = open(INPUT_FILE).read().split("\n")
    if contents[-1] == "":
        contents.pop()
    return [list(map(int, row)) for row in contents]


directions = [(0, 1), (1, 0), (0, -1), (-1, 0)]


def get_trail_count(input: List, start: Tuple[int, int]) -> int:
    q = queue.Queue()
    q.put(start)
    summits = set()
    unique_paths = 0
    while not q.empty():
        x, y = q.get()
        val = input[x][y]
        if val == 9:
            summits.add((x, y))
            unique_paths += 1
            continue
        for dx, dy in directions:
            nx, ny = x + dx, y + dy
            if (
                0 <= nx < len(input)
                and 0 <= ny < len(input[nx])
                and input[nx][ny] == val + 1
            ):
                q.put((nx, ny))
    return unique_paths, len(summits)


def part_one(input: List) -> int:
    trail_count = 0
    for i in range(len(input)):
        for j in range(len(input[i])):
            if input[i][j] == 0:
                unique_paths, summits = get_trail_count(input, (i, j))
                trail_count += summits
    return trail_count


def part_two(input: List) -> int:
    trail_count = 0
    for i in range(len(input)):
        for j in range(len(input[i])):
            if input[i][j] == 0:
                unique_paths, summits = get_trail_count(input, (i, j))
                trail_count += unique_paths
    return trail_count


if __name__ == "__main__":
    input = get_input()
    print(f"Part 1: {part_one(input)}")
    print(f"Part 2: {part_two(input)}")
