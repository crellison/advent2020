from copy import deepcopy
from collections import defaultdict
from os.path import abspath, dirname
from typing import List, Tuple, Dict, Set
import time

INPUT_FILE = abspath(dirname(abspath(__file__)) + "/../../input/2024/15-1.txt")

char_to_direction = {
    ">": (0, 1),
    "v": (1, 0),
    "<": (0, -1),
    "^": (-1, 0),
}


def get_input() -> Tuple[Dict, str, Tuple[int, int]]:
    [map, instructions] = open(INPUT_FILE).read().split("\n\n")
    map_dict = defaultdict(lambda: "#")
    start_loc = None
    for i, row in enumerate(map.split("\n")):
        for j, char in enumerate(row):
            if char == "@":
                start_loc = (i, j)
            map_dict[(i, j)] = char

    return map_dict, instructions, start_loc


def get_input_two() -> Tuple[Dict, str, Tuple[int, int]]:
    [map, instructions] = open(INPUT_FILE).read().split("\n\n")
    map_dict = defaultdict(lambda: "#")
    start_loc = None
    for i, row in enumerate(map.split("\n")):
        for j, char in enumerate(row):
            new_j = j * 2
            if char == "#" or char == ".":
                map_dict[(i, new_j)] = char
                map_dict[(i, new_j + 1)] = char
            if char == "@":
                start_loc = (i, new_j)
                map_dict[(i, new_j)] = char
                map_dict[(i, new_j + 1)] = "."
            if char == "O":
                map_dict[(i, new_j)] = "["
                map_dict[(i, new_j + 1)] = "]"

    return map_dict, instructions, start_loc


def print_map(map: Dict):
    min_i = min(map.keys(), key=lambda x: x[0])[0]
    max_i = max(map.keys(), key=lambda x: x[0])[0]
    min_j = min(map.keys(), key=lambda x: x[1])[1]
    max_j = max(map.keys(), key=lambda x: x[1])[1]
    for i in range(min_i, max_i + 1):
        for j in range(min_j, max_j + 1):
            print(map[(i, j)], end="")
        print()


def part_one(map: Dict, instructions: str, start_loc: Tuple[int, int]) -> int:
    current_loc = start_loc
    for instruction in instructions:
        if instruction not in char_to_direction:
            continue
        direction = char_to_direction[instruction]
        shift_to = current_loc
        while map[shift_to] != ".":
            next_loc = (shift_to[0] + direction[0], shift_to[1] + direction[1])
            if map[next_loc] == "#":
                shift_to = current_loc
                break
            shift_to = next_loc

        if shift_to == current_loc:
            continue

        while shift_to != current_loc:
            map[shift_to] = map[
                (shift_to[0] - direction[0], shift_to[1] - direction[1])
            ]
            shift_to = (shift_to[0] - direction[0], shift_to[1] - direction[1])

        map[current_loc] = "."
        current_loc = (current_loc[0] + direction[0], current_loc[1] + direction[1])

    return sum(i * 100 + j for i, j in map.keys() if map[(i, j)] == "O")


def part_two(map: List, instructions: str, start_loc: Tuple[int, int]) -> int:
    def get_positions_to_move(
        current_loc: Tuple[int, int], direction: str
    ) -> Set[Tuple[int, int]]:
        di, dj = char_to_direction[direction]
        positions = set()
        if direction == ">" or direction == "<":
            shift_to = current_loc
            while map[shift_to] != ".":
                positions.add(shift_to)
                next_loc = (shift_to[0] + di, shift_to[1] + dj)
                if map[next_loc] == "#":
                    return set()
                shift_to = next_loc
        else:
            leading_edge = set()
            leading_edge.add(current_loc)
            while any(map[loc] != "." for loc in leading_edge):
                # print(f"Leading edge: {leading_edge}")
                if len(leading_edge) == 0:
                    break
                positions.update(leading_edge)
                new_leading_edge = set()
                for loc in leading_edge:
                    if map[loc] == ".":
                        positions.remove(loc)
                        continue
                    next_loc = (loc[0] + di, loc[1] + dj)
                    if map[next_loc] == "#":
                        return set()
                    elif map[next_loc] == "[":
                        new_leading_edge.add((next_loc[0], next_loc[1] + 1))
                    elif map[next_loc] == "]":
                        new_leading_edge.add((next_loc[0], next_loc[1] - 1))
                    new_leading_edge.add(next_loc)
                leading_edge = new_leading_edge
            # print(f"last leading edge: {leading_edge}")

        return positions

    current_loc = start_loc

    for i, instruction in enumerate(instructions):
        # start_time = time.time()
        if instruction not in char_to_direction:
            continue
        positions_to_move = get_positions_to_move(current_loc, instruction)

        if len(positions_to_move) == 0:
            continue

        map_changes = defaultdict(lambda: "#")
        di, dj = char_to_direction[instruction]

        # for i, j in positions_to_move:
        #     if (map[(i, j)] == "]" and map[(i, j - 1)] != "[") or (
        #         map[(i, j)] == "[" and map[(i, j + 1)] != "]"
        #     ):
        #         print("=== INVALID MOVE ===")
        #         print(f"Move: {instruction} with position: {i, j}")
        #         print_map(map)
        #         print(f"Positions to move: {positions_to_move}")
        #         return -1

        for i, j in positions_to_move:
            map_changes[(i, j)] = "."
        for i, j in positions_to_move:
            map_changes[(i + di, j + dj)] = map[(i, j)]
        if len(positions_to_move) != 0:
            current_loc = (current_loc[0] + di, current_loc[1] + dj)
        # if instruction in "^v" and len(positions_to_move) > 1:
        #     print("===")
        #     print(f"Move: {instruction}")
        #     print_map(map)
        #     print(f"Positions to move: {positions_to_move}")
        for loc, val in map_changes.items():
            map[loc] = val

        # delta_for_frame = 1 / 24 - (time.time() - start_time)
        # time.sleep(delta_for_frame)
        # print_map(map)
    print_map(map)
    return sum(i * 100 + j for i, j in map.keys() if map[(i, j)] in "[")


if __name__ == "__main__":
    print(f"Part 1: {part_one(*get_input())}")
    print(f"Part 2: {part_two(*get_input_two())}")
