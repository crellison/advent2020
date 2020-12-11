from os.path import abspath, dirname
from typing import List, Tuple
from copy import deepcopy

INPUT_FILE = dirname(abspath(__file__)) + "/inputs/11-1.txt"

Board = List[List[str]]


def get_input() -> Board:
    contents = [list(line) for line in open(INPUT_FILE).read().split("\n") if line != ""]
    return contents


empty = "L"
occ = "#"
floor = "."

directions = [(dx, dy) for dx in range(-1, 2) for dy in range(-1, 2) if (dx, dy) != (0, 0)]


def step_game_of_life(board: Board) -> Board:
    new_board = deepcopy(board)
    board_x, board_y = len(board), len(board[0])
    for x in range(board_x):
        for y in range(board_y):
            occupied_seats = sum([board[x + dx][y + dy] == occ for (dx, dy) in directions])
            if board[x][y] == empty and occupied_seats == 0:
                new_board[x][y] = occ
            elif board[x][y] == occ and occupied_seats >= 4:
                new_board[x][y] = empty
    return new_board


def step_generous_game(board: Board) -> Board:
    new_board = deepcopy(board)
    for x in range(len(board)):
        for y in range(len(board[0])):
            occupied_seats = sum(
                [first_visible_is_occupied(board, (x, y), (dx, dy)) for (dx, dy) in directions]
            )
            if board[x][y] == empty and occupied_seats == 0:
                new_board[x][y] = occ
            elif board[x][y] == occ and occupied_seats >= 5:
                new_board[x][y] = empty
    return new_board


def first_visible_is_occupied(
    board: Board, starting: Tuple[int, int], direction: Tuple[int, int]
) -> str:
    board_x, board_y = range(len(board)), range(len(board[0]))
    x, y = starting
    dx, dy = direction
    check_x, check_y = x + dx, y + dy
    while check_x in board_x and check_y in board_y:
        if board[check_x][check_y] != floor:
            return board[check_x][check_y] == occ
        check_x += dx
        check_y += dy
    return False


if __name__ == "__main__":
    input = get_input()
    next_step = step_generous_game(input)

    count = 0
    while not next_step == input:
        count += 1
        input = next_step
        next_step = step_generous_game(input)

    print(f"generations: {count}")
    print(sum([char == occ for line in input for char in line]))
