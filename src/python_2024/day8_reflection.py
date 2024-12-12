from copy import deepcopy
from collections import defaultdict
from os.path import abspath, dirname
from typing import List, Tuple

INPUT_FILE = abspath(dirname(abspath(__file__)) + "/../../input/2024/8-1.txt")


def get_input() -> Tuple[defaultdict, int, int]:
    contents = open(INPUT_FILE).read().split("\n")
    if contents[-1] == "":
        contents.pop()
    node_map = defaultdict(list)
    for i, line in enumerate(contents):
        for j, char in enumerate(line):
            if char == ".":
                continue
            node_map[char].append((i, j))

    return node_map, range(len(contents)), range(len(contents[0]))


def part_one(input: Tuple[defaultdict, range, range]) -> int:
    node_map, i_range, j_range = input
    anti_nodes = set()
    for positions in node_map.values():
        for m in range(len(positions)):
            for n in range(m + 1, len(positions)):
                # print(f"considering {positions[m]} and {positions[n]}")
                di = positions[m][0] - positions[n][0]
                dj = positions[m][1] - positions[n][1]
                r1_i, r1_j = (positions[m][0] + di, positions[m][1] + dj)
                r2_i, r2_j = (positions[n][0] - di, positions[n][1] - dj)
                # print(f"reflected to {r1_i}, {r1_j} and {r2_i}, {r2_j}")

                if r1_i in i_range and r1_j in j_range:
                    anti_nodes.add((r1_i, r1_j))
                if r2_i in i_range and r2_j in j_range:
                    anti_nodes.add((r2_i, r2_j))

    return len(anti_nodes)


def part_two(input: Tuple[defaultdict, range, range]) -> int:
    node_map, i_range, j_range = input
    anti_nodes = set()
    for positions in node_map.values():
        for m in range(len(positions)):
            for n in range(m + 1, len(positions)):
                # print(f"considering {positions[m]} and {positions[n]}")
                di = positions[m][0] - positions[n][0]
                dj = positions[m][1] - positions[n][1]
                for i in range(len(i_range) * len(j_range)):
                    r1_i, r1_j = (positions[m][0] + di * i, positions[m][1] + dj * i)
                    if r1_i in i_range and r1_j in j_range:
                        anti_nodes.add((r1_i, r1_j))
                    else:
                        break
                for i in range(len(i_range) * len(j_range)):
                    r2_i, r2_j = (positions[n][0] - di * i, positions[n][1] - dj * i)

                    if r2_i in i_range and r2_j in j_range:
                        anti_nodes.add((r2_i, r2_j))
                    else:
                        break

    return len(anti_nodes)


if __name__ == "__main__":
    input = get_input()
    print(f"Part 1: {part_one(input)}")
    # 356 too low
    print(f"Part 2: {part_two(input)}")
