from math import cos, sin, radians
from os.path import abspath, dirname
from typing import List, Tuple

INPUT_FILE = dirname(abspath(__file__)) + "/inputs/12-1.txt"


def get_input() -> List:
    contents = [l for l in open(INPUT_FILE).read().split("\n") if l != ""]
    return contents


angles = {
    0: (1, 0),
    90: (0, 1),
    180: (-1, 0),
    270: (0, -1),
    "E": (1, 0),
    "N": (0, 1),
    "W": (-1, 0),
    "S": (0, -1),
}
range_2 = range(2)


def move_ship(input: List[str]) -> Tuple[int, int]:
    angle = 0
    pos = [0, 0]
    for line in input:
        cmd, num = line[0], int(line[1:])
        if cmd in "LR":
            angle = (360 + (-num if cmd == "R" else num)) % 360
        else:
            delta = [num * x for x in angles[angle if cmd == "F" else cmd]]
            pos = [pos[i] + delta[i] for i in range_2]
    return pos


def rotate(waypoint: List[int], angle: float) -> List[int]:
    """Rotates an array of len 2 by radians defined in angle and rounds to int"""
    x = round(cos(angle) * waypoint[0] - sin(angle) * waypoint[1])
    y = round(sin(angle) * waypoint[0] + cos(angle) * waypoint[1])
    return [x, y]


def move_waypoint_and_ship(input: List[str]) -> Tuple[int, int]:
    pos, wp = [0, 0], [10, 1]
    for line in input:
        cmd, num = line[0], int(line[1:])
        if cmd in "LR":
            angle = (360 + (-num if cmd == "R" else num)) % 360
            wp = rotate(wp, radians(angle))
        elif cmd == "F":
            pos = [pos[i] + wp[i] * num for i in range_2]
        else:
            delta = [num * x for x in angles[cmd]]
            wp = [wp[i] + delta[i] for i in range_2]
    return pos


def manhattan_distance(coords):
    return sum(abs(x) for x in coords)


if __name__ == "__main__":
    input = get_input()
    ship_coords = move_ship(input)
    print(manhattan_distance(ship_coords))
    final_coords = move_waypoint_and_ship(input)
    print(manhattan_distance(final_coords))
