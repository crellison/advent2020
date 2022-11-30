from os.path import abspath, dirname
from typing import Dict, Tuple

INPUT_FILE = dirname(abspath(__file__)) + "/inputs/17-1.txt"

delta = range(-1, 2)
directions_d3 = [
    (dx, dy, dz) for dx in delta for dy in delta for dz in delta if (dx, dy, dz) != (0, 0, 0)
]
directions_d4 = [
    (dx, dy, dz, dw)
    for dx in delta
    for dy in delta
    for dz in delta
    for dw in delta
    if (dx, dy, dz, dw) != (0, 0, 0, 0)
]


def get_input() -> Dict[Tuple, bool]:
    contents = [list(line) for line in open(INPUT_FILE).read().split("\n") if line != ""]
    board = {
        (x, y, 0, 0): True
        for x in range(len(contents))
        for y in range(len(contents[x]))
        if contents[x][y] == "#"
    }

    return board


def step_field(field: Dict[Tuple, bool]) -> Dict[Tuple, bool]:
    new_field = dict()
    entries_to_check = set()
    for coord, value in field.items():
        if value:
            x, y, z, w = coord
            entries_to_check.add(coord)
            entries_to_check.update(
                (x + dx, y + dy, z + dz, w + dw) for dx, dy, dz, dw in directions_d4
            )
    for entry in entries_to_check:
        x, y, z, w = entry
        occupied_neighbors = sum(
            (x + dx, y + dy, z + dz, w + dw) in field for dx, dy, dz, dw in directions_d4
        )
        if entry in field and occupied_neighbors in [2, 3]:
            new_field[entry] = True
        elif entry not in field and occupied_neighbors == 3:
            new_field[entry] = True
    return new_field


if __name__ == "__main__":
    input = get_input()
    for i in range(6):
        input = step_field(input)
    print(sum(input.values()))
