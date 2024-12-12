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
    return len(visited)


def part_two(start: Tuple[int, int], obstacles: defaultdict) -> int:
    possible_obstacles = [
        (i, j)
        for i in range(min(obstacles.keys()) + 1, max(obstacles.keys()))
        for j in range(min(obstacles[i].keys()) + 1, max(obstacles[i].keys()))
        if j not in obstacles[i] and (i, j) != start
    ]
    valid_obstacle_count = 0

    def check_for_loop(obstacles):
        visited = set()
        visited.add(start)
        dir_index = 0
        pos = start
        possible_loop = False
        for i in range(1, 1000):  # no infinite loops
            next_obstacle, next_pos = get_next_pos(
                pos, obstacles, directions[dir_index]
            )
            distance_from_current = abs(next_obstacle[0] - pos[0]) + abs(
                next_obstacle[1] - pos[1]
            )
            if next_pos in visited and distance_from_current > 1:
                if not possible_loop:
                    # need to validate that the step after next is in visited
                    possible_loop = True
                else:
                    # print(f"loop found after step {i} with next position {next_pos}")
                    return True

            # print(f"pos: {pos}, next_pos: {next_pos}, next_obstacle: {next_obstacle}")
            di, dj = directions[dir_index]
            dir_index = (dir_index + 1) % 4

            for i in range(distance_from_current):
                visited.add((pos[0] + di * i, pos[1] + dj * i))
            pos = next_pos
            if (
                next_obstacle[0] == min(obstacles.keys())
                or next_obstacle[0] == max(obstacles.keys())
                or next_obstacle[1] == min(obstacles[-1].keys())
                or next_obstacle[1] == max(obstacles[-1].keys())
            ):
                return False

    for i, j in possible_obstacles:
        obstacles[i][j] = True
        if check_for_loop(obstacles):
            # print(f"loop found with obstacle at {i}, {j}")
            valid_obstacle_count += 1
        del obstacles[i][j]

    return valid_obstacle_count


if __name__ == "__main__":
    start, obstacles = get_input()
    print(f"Part 1: {part_one(start, obstacles)}")
    print(f"Part 2: {part_two(start, obstacles)}")
    # 900, 901 too low
    # 1578 too high
    # 1577 too high
