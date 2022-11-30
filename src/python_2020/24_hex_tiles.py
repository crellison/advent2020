import re
from collections import defaultdict
from os.path import abspath, dirname
from typing import Dict, List, Set, Tuple

INPUT_FILE = dirname(abspath(__file__)) + "/inputs/24-1.txt"

# dirs[x] = (regex match, standard plane delta)
dirs = {
    "e": (r"(?<![n|s])(?P<east>e)", (1.0, 0.0)),
    "w": (r"(?<![n|s])(?P<west>w)", (-1.0, 0.0)),
    "nw": (r"(nw)", (-0.5, 0.75)),
    "sw": (r"(sw)", (-0.5, -0.75)),
    "ne": (r"(ne)", (0.5, 0.75)),
    "se": (r"(se)", (0.5, -0.75)),
}
dir_keys = dirs.keys()


def get_input() -> List:
    contents = [
        {direction: len(re.findall(dir_val[0], l)) for direction, dir_val in dirs.items()}
        for l in open(INPUT_FILE).read().split("\n")
        if l != ""
    ]
    return contents


def landing_location(path: Dict[str, int]) -> Tuple[float, float]:
    all_moves = [
        move for direction, dir_val in dirs.items() for move in [dir_val[1]] * path[direction]
    ]
    final_location = tuple(sum(x) for x in zip(*all_moves))
    return final_location


def flip_tiles(tiles: List[Dict[str, int]]) -> Set[Tuple[float, float]]:
    flipped = set()
    for tile in tiles:
        final_location = landing_location(tile)
        if final_location in flipped:
            flipped.remove(final_location)
        else:
            flipped.add(final_location)
    return flipped


def get_adjacent(coordinate: Tuple[float, float]) -> List[Tuple[float, float]]:
    return [tuple(sum(x) for x in zip(coordinate, dir_val[1])) for dir_val in dirs.values()]


def iterate_tiles(tile_board: Set[Tuple[float, float]]) -> Set[Tuple[float, float]]:
    """Flips all tiles on the board according to rules and returns the new board

    the tile board is a dict of coordinates of the centers of hexagons on a grid
    the values of the board indicate whether or not the tile in question is black

    black tiles: True
    white tiles: False

    black tiles stay black if they have 1 or 2 black neighbors
    white tiles turn black if they have 2 black neighbors
    """
    new_board = set()
    tiles_to_check = set(tile_board)
    checked_tiles = set()
    num_checked = 0
    while tiles_to_check:
        num_checked += 1
        coordinate = tiles_to_check.pop()
        checked_tiles.add(coordinate)
        adjacent = get_adjacent(coordinate)
        count_adjacent = 0
        for location in adjacent:
            if location in tile_board:
                count_adjacent += 1
            if coordinate in tile_board and location not in checked_tiles:
                tiles_to_check.add(location)

        if (coordinate in tile_board and count_adjacent in [1, 2]) or (
            coordinate not in tile_board and count_adjacent == 2
        ):
            new_board.add(coordinate)
    return new_board


if __name__ == "__main__":
    input = get_input()
    tiles = flip_tiles(input)

    print(f"initially {len(tiles)} black tiles")

    for i in range(1000):
        tiles = iterate_tiles(tiles)

    print(f"after 100 days: {len(tiles)} black tiles")
