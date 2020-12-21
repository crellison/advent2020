import math
from copy import deepcopy
from collections import defaultdict
from os.path import abspath, dirname
from typing import Any, Dict, List, Set, Tuple
from functools import reduce
import re


INPUT_FILE = dirname(abspath(__file__)) + "/inputs/20-1.txt"

CONTENTS = "contents"
BORDERS = "borders"

Tile_Type = Dict[int, Dict[str, List[str]]]
Matching_Borders = Dict[str, List[Tuple[int, int, bool]]]
Neighbors = Dict[int, Dict]
position_deltas: Dict[int, Tuple[int, int]] = {
    0: (0, 1),
    1: (1, 0),
    2: (0, -1),
    3: (-1, 0),
}


def get_input() -> Tile_Type:
    tiles = dict()
    tiles_raw = [tile.split("\n") for tile in open(INPUT_FILE).read().split("\n\n") if tile != ""]
    for raw_tile in tiles_raw:
        tile_id = int(raw_tile[0][5:-1])
        tile_contents = [l for l in raw_tile[1:] if l != ""]
        borders = get_borders(tile_contents)
        tiles[tile_id] = {CONTENTS: tile_contents, BORDERS: borders}
    return tiles


def get_borders(tile: List[str]):
    """Returns the north, east, south, and west edges.

    Parsed all borders reading clockwise around the tile.
    """
    north, south = tile[0], tile[-1][::-1]
    west, east = reduce(reduce_left_and_right, tile, ["", ""])
    return [north, east, south, west]


def reduce_left_and_right(acc: List[str], tile_row: str):
    """Accumulates the left and right values of rows in acc, respectively.

    to be used with functools.reduce
    """
    acc[0] = tile_row[0] + acc[0]
    acc[1] += tile_row[-1]
    return acc


def find_matching_borders(tiles: Tile_Type) -> Matching_Borders:
    """Assembles a dict of borders with the tiles that share those borders.

    Of the form: { <border_string>: list(tuple(tile_id, tile_side_index))}
    """
    matching_borders = defaultdict(list)
    for tile_id, tile in tiles.items():
        for index, border in enumerate(tile[BORDERS]):
            # first border entered in will always be unflipped
            if border[::-1] in matching_borders:
                matching_borders[border[::-1]].append((tile_id, index))
            else:
                matching_borders[border].append((tile_id, index))
    return matching_borders


def get_corners(matching_borders: Matching_Borders) -> Set[int]:
    """Finds the corners given the matching borders"""
    edges, corners = set(), set()
    for matching_tiles in matching_borders.values():
        if len(matching_tiles) == 1:
            tile_id = matching_tiles[0][0]
            if tile_id in edges:
                corners.add(tile_id)
            else:
                edges.add(tile_id)
    return corners


def build_neighbors(matching_borders: Matching_Borders) -> Neighbors:
    """Builds an adjacency dictionary of neighbors

    Of the form: { <tile_id>: { <side_id>: tuple(tile_id, side_id) } }
    """
    neighbor_dict = defaultdict(dict)
    for match in matching_borders.values():
        if len(match) == 1:
            continue
        t1_id, t1_side = match[0]
        t2_id, t2_side = match[1]
        neighbor_dict[t1_id][t1_side] = (t2_id, t2_side)
        neighbor_dict[t2_id][t2_side] = (t1_id, t1_side)
    return neighbor_dict


def flip_and_rotate(tile: List[str], adjacent_to: int, own_side: int, flipped: bool):
    """Flips and rotates a tile while also calculating an adjacency map for the neighbor map"""
    new_borders = [0, 1, 2, 3]
    new_contents = deepcopy(tile)

    # rotate into position
    adjacency_dict = {0: 2, 1: 3, 2: 0, 3: 1}
    target_index = adjacency_dict[adjacent_to]
    current_index = new_borders.index(own_side)
    if target_index != current_index:
        shift = current_index - target_index
        # print(f"expecting {current_index} to match {target_index}. shifting {shift} units")
        rotations = (4 - shift) % 4
        # print(f"rotating {rotations} times")
        new_borders = new_borders[shift:] + new_borders[:shift]
        new_contents = rotate_right(new_contents, rotations)

    if flipped:
        if adjacent_to % 2 == 0:  # horizontal flip
            # print('flipping horizontal')
            north, south = new_borders[1], new_borders[3]
            new_borders[3] = north
            new_borders[1] = south
            new_contents = flip_tile(new_contents, True)
        else:
            # print('flipping vertical')
            north, south = new_borders[0], new_borders[2]
            new_borders[2] = north
            new_borders[0] = south
            new_contents = flip_tile(new_contents, False)

    return new_borders, new_contents


def flip_tile(tile: List[str], horizontal: bool) -> List[str]:
    if horizontal:
        return [l[::-1] for l in tile]
    return tile[::-1]


def rotate_right(tile_contents: List[str], num_rotations):
    """Rotates tile contents 90 degrees * num_rotations"""
    max_y, max_x = len(tile_contents) - 1, len(tile_contents[0]) - 1
    if num_rotations == 0:
        return tile_contents
    broken_tile = [list(line) for line in tile_contents]
    new_tile = deepcopy(broken_tile)
    for x in range(max_x + 1):
        for y in range(max_y + 1):
            if num_rotations == 1:
                new_tile[x][max_y - y] = broken_tile[y][x]
            elif num_rotations == 2:
                new_tile[max_y - y][max_x - x] = broken_tile[y][x]
            elif num_rotations == 3:
                new_tile[max_x - x][y] = broken_tile[y][x]

    return ["".join(line) for line in new_tile]


def build_board(matching_borders: Matching_Borders, tiles: Tile_Type) -> Dict:
    """Builds the board by taking matching borders and creating an arrangement of tiles in space

    Chooses an arbitrary starting tile and decides that its orientation is correct. All other tiles
    are made to conform to that anchor tile. As a tile is checked, it is first rotated and flipped
    as necessary to align the tile to the grid. Then the tile updates the new expected location of
    the tiles bordering it in the neighbor_dict. These neighbors are then added to the grid if they
    have not already been checked.
    """
    neighbor_dict = build_neighbors(matching_borders)
    anchor_id = list(matching_borders.values())[0][0][0]  # choose arbitrary tile as anchor
    board_field = {(0, 0): tiles[anchor_id][CONTENTS], anchor_id: (0, 0)}  ## initialize board

    visited_tiles = set()
    tiles_to_visit = [(anchor_id, (0, 0))]
    while len(tiles_to_visit) != 0:
        current_tile, position = tiles_to_visit.pop()
        visited_tiles.add(current_tile)
        # get the borders from the stored version of the tile on the board
        current_borders = get_borders(board_field[position])

        # update board with neighbors
        for side in [0, 1, 2, 3]:
            # ignore if no entry on that side or if we have already seen the tile
            if neighbor_dict[current_tile].get(side) is None:
                continue
            neighbor_id, neighbor_side = neighbor_dict[current_tile][side]
            if neighbor_id in board_field:
                continue

            should_flip = current_borders[side] == tiles[neighbor_id][BORDERS][neighbor_side]

            # orient tile to match current tile
            new_borders, new_contents = flip_and_rotate(
                tiles[neighbor_id][CONTENTS], side, neighbor_side, should_flip
            )
            neighbor_pos = tuple(sum(x) for x in zip(position, position_deltas[side]))
            if neighbor_pos in board_field:
                raise Exception(f"position {neighbor_pos} exists: {neighbor_id} cannot overwrite")

            # add neighbors to field
            board_field[neighbor_pos] = new_contents
            board_field[neighbor_id] = neighbor_pos

            if neighbor_id not in visited_tiles:
                tiles_to_visit.append((neighbor_id, neighbor_pos))

            # update neighbors with new positions after rotation
            new_neighbors = {
                new_index: neighbor_dict[neighbor_id].get(old_index, None)
                for new_index, old_index in enumerate(new_borders)
            }
            for key, value in new_neighbors.items():
                neighbor_dict[neighbor_id][key] = value

    return board_field


def stitch_together_board(board_field: Dict) -> List[str]:
    """Stitches a board together from a field with arbitrary coordinates

    Returns tiles as a mega-tile with all borders removed.
    """
    coordinates = [e for e in board_field.keys() if type(e) is tuple]
    max_x, max_y = tuple(max(x) for x in zip(*coordinates))
    min_x, min_y = tuple(min(x) for x in zip(*coordinates))

    complete_board = []
    for y in range(max_y, min_y - 1, -1):
        board_row = []
        for x in range(min_x, max_x + 1):
            stripped_tile = strip_border(board_field[(x, y)])
            if len(board_row) == 0:
                board_row = stripped_tile
            else:
                for index, line in enumerate(stripped_tile):
                    board_row[index] += line
        complete_board.extend(board_row)
    return complete_board


def strip_border(tile: List[str]) -> List[str]:
    return [line[1:-1] for line in tile[1:-1]]


def count_sea_monsters(board: List[str]) -> int:
    """Counts number of sea monsters in a board"""
    monster_top = r"..................#."
    monster_middle = r"#....##....##....###"
    monster_bottom = r".#..#..#..#..#..#..."
    monster_count = 0
    for index in range(1, len(board) - 1):
        line = board[index]
        offset = 0
        maybe_monster = re.search(monster_middle, line[::])
        while maybe_monster is not None:
            start = maybe_monster.start()
            end = maybe_monster.end()
            found_monster = bool(
                re.search(monster_top, board[index - 1][offset + start : offset + end])
                and re.search(monster_bottom, board[index + 1][offset + start : offset + end])
            )

            offset += end
            monster_count += found_monster
            maybe_monster = re.search(monster_middle, line[offset:])
    return monster_count


if __name__ == "__main__":
    tiles = get_input()
    print(f"tiles make a square with dim: {math.sqrt(len(tiles))}")
    matching_borders = find_matching_borders(tiles)

    corners = get_corners(matching_borders)
    print(f"corners: {corners}")
    print(f"corner product: {math.prod(corners)}")

    board_field = build_board(matching_borders, tiles)
    complete_board = stitch_together_board(board_field)

    monster_count = 0
    for i in range(4):
        tmp_board = rotate_right(complete_board, i)
        tmp_count = max(count_sea_monsters(tmp_board), count_sea_monsters(flip_tile(tmp_board, True)))
        if tmp_count != 0:
            monster_count = tmp_count

    hash_count = sum(len(line.replace(".", "")) for line in complete_board)
    roughness = hash_count - monster_count * 15 # 15 `#` per monster
    print(f"roughness is {roughness} with {monster_count} monsters")
