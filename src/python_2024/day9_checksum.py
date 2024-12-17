from copy import deepcopy
from collections import defaultdict
from os.path import abspath, dirname
from typing import List
import heapq

INPUT_FILE = abspath(dirname(abspath(__file__)) + "/../../input/2024/9-1.txt")


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


"""
blocks [(id, start, len)]
possible to keep track of empty blocks by size?
empty_blocks {
    size: min_queue(start_index)
}


"""


def part_two(input: str) -> int:
    """
    move files to forwardmost position they fit from back to front without breaking up files
    """
    files = []  # (id, start_position, length)
    empty_blocks = defaultdict(list)  # (length, start_position) managed by heapq
    block_position = 0
    for i, num in enumerate(input):
        block_len = int(num)
        if block_len != 0:
            if i % 2 == 0:
                files.append((i // 2, block_position, block_len))
            else:
                heapq.heappush(empty_blocks[block_len], block_position)
        block_position += block_len

    for i in range(len(files) - 1, -1, -1):
        id, start_position, file_len = files[i]
        relocation_slot_size = min(
            (
                x
                for x in empty_blocks.keys()
                if x >= file_len and len(empty_blocks[x]) > 0
            ),
            default=-1,
            key=lambda x: empty_blocks[x][0],
        )
        if relocation_slot_size == -1:
            continue
        relocation_slot_start = heapq.heappop(empty_blocks[relocation_slot_size])
        if relocation_slot_start >= start_position:
            continue
        # print(f"moving file {id} from {start_position} to {relocation_slot_start}")
        files[i] = (id, relocation_slot_start, file_len)
        heapq.heappush(
            empty_blocks[relocation_slot_size - file_len],
            relocation_slot_start + file_len,
        )

    checksum = 0
    for id, start_position, file_len in files:
        for i in range(file_len):
            checksum += id * (start_position + i)

    return checksum


if __name__ == "__main__":
    input = get_input()
    print(f"Part 1: {part_one(input)}")
    print(f"Part 2: {part_two(input)}")
