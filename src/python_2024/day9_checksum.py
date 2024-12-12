from copy import deepcopy
from collections import defaultdict
from os.path import abspath, dirname
from typing import List

INPUT_FILE = abspath(dirname(abspath(__file__)) + "/../../input/2024/9-test.txt")


def get_input() -> str:
    contents = open(INPUT_FILE).read().split("\n")
    return contents[0]


"""
3249501345

num index % 2 == 0 -> file with id index // 2 and length num
num index % 2 == 1 -> empty space with length num
"""


def part_one(input: str) -> int:
    """
    move all file blocks to the front of the input from back to front by block
    """
    block_position = 0
    tail_cursor = len(input) - 1 if len(input) % 2 == 1 else len(input) - 2
    tail_blocks = int(input[tail_cursor])
    checksum = 0
    for i, num in enumerate(input):
        block_len = int(num)
        if i == tail_cursor:
            for _ in range(tail_blocks):
                checksum += block_position * (tail_cursor // 2)
                block_position += 1
            break
        if i % 2 == 0 and i != tail_cursor:  # processing a file
            for _ in range(block_len):
                checksum += block_position * (i // 2)
                block_position += 1
        else:  # processing empty space
            for _ in range(block_len):
                if tail_blocks == 0:
                    tail_cursor -= 2
                    tail_blocks = int(input[tail_cursor])
                checksum += block_position * (tail_cursor // 2)
                tail_blocks -= 1
                block_position += 1
    return checksum


def part_two(input: str) -> int:
    """
    move files to forwardmost position they fit from back to front without breaking up files
    """
    block_position = 0
    tail_cursor = len(input) - 1 if len(input) % 2 == 1 else len(input) - 2
    tail_blocks = int(input[tail_cursor])
    checksum = 0
    for i, num in enumerate(input):
        block_len = int(num)
        if i == tail_cursor:
            for _ in range(tail_blocks):
                checksum += block_position * (tail_cursor // 2)
                block_position += 1
            break
        if i % 2 == 0 and i != tail_cursor:  # processing a file
            for _ in range(block_len):
                checksum += block_position * (i // 2)
                block_position += 1
        else:  # processing empty space
            while 
            for _ in range(block_len):
                if tail_blocks == 0:
                    tail_cursor -= 2
                    tail_blocks = int(input[tail_cursor])
                checksum += block_position * (tail_cursor // 2)
                tail_blocks -= 1
                block_position += 1
    return checksum


if __name__ == "__main__":
    input = get_input()
    print(f"Part 1: {part_one(input)}")
    print(f"Part 2: {part_two(input)}")
