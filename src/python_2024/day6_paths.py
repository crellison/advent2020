import sys
from copy import deepcopy
from collections import defaultdict
from os.path import abspath, dirname
from typing import List, Tuple

INPUT_FILE = abspath(dirname(abspath(__file__)) + "/../../input/2024/6-1.txt")

# order of directions (row, col) in up, right, down, left
directions = [(-1, 0), (0, 1), (1, 0), (0, -1)]


def get_input():
    start = (-1, -1)
    obstacles = defaultdict(lambda: defaultdict(bool))
    contents = open(INPUT_FILE).read().split("\n")
    for j in range(-1, len(contents[0]) + 1):
        obstacles[-1][j] = True
        obstacles[len(contents)][j] = True
    for i, line in enumerate(contents):
        obstacles[i][-1] = True
        obstacles[i][len(line)] = True
        for j, char in enumerate(line):
            if char == "^":
                start = (i, j)
            elif char == "#":
                obstacles[i][j] = True

    if start == (-1, -1):
        raise ValueError("Start not found")
    return start, obstacles


def get_next_pos(pos: Tuple[int, int], obstacles, direction: str):
    possible_positions = (
        [
            (pos[0], x)
            for x in obstacles[pos[0]].keys()
            if (x > pos[1] if direction[1] == 1 else x < pos[1])
        ]
        if direction[0] == 0
        else [
            (x, pos[1])
            for x in obstacles.keys()
            if pos[1] in obstacles[x]
            and (x > pos[0] if direction[0] == 1 else x < pos[0])
        ]
    )
    next_obstacle = min(
        possible_positions, key=lambda x: abs(x[0] - pos[0]) + abs(x[1] - pos[1])
    )
    # back up one step to not hit the obstacle
    return (
        next_obstacle,
        (next_obstacle[0] - direction[0], next_obstacle[1] - direction[1]),
    )


def part_one(start: Tuple[int, int], obstacles: defaultdict) -> int:
    visited = set()
    visited.add(start)
    dir_index = 0
    pos = start
    for i in range(1000):  # no infinite loops
        next_obstacle, next_pos = get_next_pos(pos, obstacles, directions[dir_index])
        # print(f"pos: {pos}, next_pos: {next_pos}, next_obstacle: {next_obstacle}")
        di, dj = directions[dir_index]
        for i in range(abs(next_obstacle[0] - pos[0]) + abs(next_obstacle[1] - pos[1])):
            visited.add((pos[0] + di * i, pos[1] + dj * i))
        pos = next_pos
        dir_index = (dir_index + 1) % 4
        if (
            next_obstacle[0] == min(obstacles.keys())
            or next_obstacle[0] == max(obstacles.keys())
            or next_obstacle[1] == min(obstacles[-1].keys())
            or next_obstacle[1] == max(obstacles[-1].keys())
        ):
            break
    return len(visited), visited


def part_two(
    start: Tuple[int, int], obstacles: defaultdict, obstacle_locations: set
) -> int:
    valid_obstacle_count = 0
    i_range = range(max(obstacles.keys()))
    j_range = range(max(obstacles[0].keys()))

    def check_for_loop(obstacles, new_obstacle):
        moves = set()
        dir_index = 0
        pos = start
        for i in range(1, 1000):  # no infinite loops
            next_obstacle, next_pos = get_next_pos(
                pos, obstacles, directions[dir_index]
            )
            next_move = f"{pos}|{next_pos}"
            if next_move in moves and next_obstacle == new_obstacle:
                return True
            moves.add(next_move)

            # print(f"pos: {pos}, next_pos: {next_pos}, next_obstacle: {next_obstacle}")
            dir_index = (dir_index + 1) % 4
            pos = next_pos
            if next_obstacle[0] not in i_range or next_obstacle[1] not in j_range:
                return False

    for i, j in obstacle_locations:
        if (i, j) == start:
            continue
        obstacles[i][j] = True
        if check_for_loop(obstacles, (i, j)):
            # print(f"loop found with obstacle at {i}, {j}")
            valid_obstacle_count += 1
        del obstacles[i][j]

    return valid_obstacle_count


if __name__ == "__main__":
    start, obstacles = get_input()
    part_one_result, visited = part_one(start, obstacles)
    print(f"Part 1: {part_one_result}")
    print(f"Part 2: {part_two(start, obstacles, visited)}")
    # 900, 901 too low
    # 1578 too high
    # 1577 too high
    # 1504 - wrong
    # 1400 - wrong
