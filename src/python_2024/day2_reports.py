from copy import deepcopy
from collections import defaultdict
from os.path import abspath, dirname
from typing import List

INPUT_FILE = abspath(dirname(abspath(__file__)) + "/../../input/2024/2-1.txt")


def get_input() -> List[List[int]]:
    contents = open(INPUT_FILE).read().split("\n")
    if contents[-1] == "":
        contents.pop()
    return [list(map(int, line.split())) for line in contents]


def row_to_deltas(row: List[int]) -> List[int]:
    return [row[i + 1] - row[i] for i in range(len(row) - 1)]


def check_deltas(deltas: List[int]) -> bool:
    return all(1 <= d <= 3 for d in deltas) or all(-3 <= d <= -1 for d in deltas)


def part_one(input: List[List[int]]) -> int:
    safe_rows = 0
    for row in input:
        if check_deltas(row_to_deltas(row)):
            safe_rows += 1
            continue
    return safe_rows


def part_two(input: List[List[int]]) -> int:
    safe_rows = 0
    for row in input:
        deltas = row_to_deltas(row)
        if check_deltas(deltas):
            safe_rows += 1
            continue
        # we have one potentially one bad delta
        # a0, a1, a2, a3 -> d0 = a1 - a0, d1 = a2 - a1, d2 = a3 - a2
        # bad delta means either eliminate a(i) or a(i+1)

        direction = sum(1 if d > 0 else -1 if d < 0 else 0 for d in deltas)
        if direction == 0:
            continue  # too many different deltas
        expected_values = [x * (direction // abs(direction)) for x in range(1, 4)]

        bad_delta_index = next(
            x for x in range(len(deltas)) if deltas[x] not in expected_values
        )

        left_omit = row[:bad_delta_index] + row[bad_delta_index + 1 :]
        right_omit = row[: bad_delta_index + 1] + row[bad_delta_index + 2 :]

        if check_deltas(row_to_deltas(left_omit)) or check_deltas(
            row_to_deltas(right_omit)
        ):
            safe_rows += 1

    return safe_rows


if __name__ == "__main__":
    input = get_input()
    print(f"Part One: {part_one(input)}")
    print(f"Part Two: {part_two(input)}")
