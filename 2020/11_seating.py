from os.path import abspath, dirname
from typing import List, Tuple
from copy import deepcopy

INPUT_FILE = dirname(abspath(__file__)) + "/inputs/11-1.txt"

Board = List[List[str]]


def get_input() -> Board:
    contents = [list(line) for line in open(INPUT_FILE).read().split("\n") if line != ""]
    return contents


empty, occ, floor = "L", "#", "."
directions = [(dx, dy) for dx in range(-1, 2) for dy in range(-1, 2) if (dx, dy) != (0, 0)]


def step_game_of_life(board: Board) -> Tuple[Board, bool]:
    new_board = deepcopy(board)
    changed = False
    x_range, y_range = range(len(board)), range(len(board[0]))
    for x in x_range:
        for y in y_range:
            occupied_seats = sum(
                [
                    board[x + dx][y + dy] == occ
                    for (dx, dy) in directions
                    if x + dx in x_range and y + dy in y_range
                ]
            )
            if board[x][y] == empty and occupied_seats == 0:
                new_board[x][y] = occ
                changed = True
            elif board[x][y] == occ and occupied_seats >= 4:
                new_board[x][y] = empty
                changed = True
    return new_board, changed


def step_generous_game(board: Board) -> Tuple[Board, bool]:
    new_board = deepcopy(board)
    changed = False
    x_range, y_range = range(len(board)), range(len(board[0]))
    for x in x_range:
        for y in y_range:
            occupied_seats = sum(
                [first_visible_is_occupied(board, x, y, dx, dy) for (dx, dy) in directions]
            )
            if board[x][y] == empty and occupied_seats == 0:
                new_board[x][y] = occ
                changed = True
            elif board[x][y] == occ and occupied_seats >= 5:
                new_board[x][y] = empty
                changed = True
    return new_board, changed


def first_visible_is_occupied(board: Board, x: int, y: int, dx: int, dy: int) -> str:
    board_x, board_y = range(len(board)), range(len(board[0]))
    check_x, check_y = x + dx, y + dy
    while check_x in board_x and check_y in board_y:
        if board[check_x][check_y] != floor:
            return board[check_x][check_y] == occ
        check_x += dx
        check_y += dy
    return False


if __name__ == "__main__":
    board = get_input()
    board, changed = step_generous_game(board)

    count = 0
    while changed == True:
        count += 1
        board, changed = step_generous_game(board)

    print(f"generations: {count}")
    print(sum([char == occ for line in board for char in line]))
