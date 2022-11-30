from os.path import abspath, dirname
from re import match
from typing import List, Tuple

INPUT_FILE = dirname(abspath(__file__)) + "/inputs/5-1.txt"


def get_input():
    contents = open(INPUT_FILE).read().split("\n")
    return contents


def parse_seat(seat_code: str) -> Tuple[int, int]:
    row_num = int(seat_code[:7].replace("F", "0").replace("B", "1"), 2)
    seat_num = int(seat_code[7:].replace("L", "0").replace("R", "1"), 2)
    return (row_num, seat_num)


def seat_id(seat: Tuple[int, int]) -> int:
    return seat[0] * 8 + seat[1]


def highest_seat_id(seat_ids: List[str]) -> int:
    max_code = max(seat_ids)
    return max_code


def find_missing_seat(seat_ids: List[str]) -> int:
    seat_ids.sort()
    x = 0
    while seat_ids[x + 1] != seat_ids[x] + 2:
        x += 1
        if x == len(seat_ids):
            raise Exception("out of range")
    return seat_ids[x] + 1


if __name__ == "__main__":
    seat_ids = [seat_id(parse_seat(code)) for code in get_input() if code != ""]
    # print(highest_seat_id(seat_ids))
    # print(seat_ids)
    print(find_missing_seat(seat_ids))
