from copy import deepcopy
from collections import defaultdict
from os.path import abspath, dirname
from typing import List, Callable
import sys

sys.setrecursionlimit(10000)

INPUT_FILE = abspath(dirname(abspath(__file__)) + "/../../input/2024/11-1.txt")


class LinkedListNode:
    def __init__(self, value: int = None, next: "LinkedListNode" = None):
        self.value = value
        self.next = next

    def __str__(self):
        if self.next is None:
            return f"{self.value}"
        return f"{self.value} -> {self.next.__str__()}"

    def __len__(self):
        if self.value is None:
            return 0
        if self.next is None:
            return 1
        return 1 + len(self.next)

    def set_value(self, value: int):
        self.value = value

    def set_next(self, next: "LinkedListNode"):
        self.next = next


def get_input() -> List[int]:
    contents = open(INPUT_FILE).read().split("\n")
    if contents[-1] == "":
        contents.pop()
    return list(map(int, contents[0].split(" ")))
    # head = LinkedListNode()
    # for value in contents[0].split(" ")[::-1]:
    #     head.set_value(int(value))
    #     prev = LinkedListNode()
    #     prev.set_next(head)
    #     head = prev
    # return head.next


def blink_stones(node: LinkedListNode) -> LinkedListNode:
    if node is None or node.value is None:
        return node
    string_value = str(node.value)
    if node.value == 0:
        node.set_value(1)
        blink_stones(node.next)
    elif len(string_value) % 2 == 0:
        node.set_value(int(string_value[: len(string_value) // 2]))
        node.set_next(
            LinkedListNode(int(string_value[len(string_value) // 2 :]), node.next)
        )
        blink_stones(node.next.next)
    else:
        node.set_value(node.value * 2024)
        blink_stones(node.next)
    return node


def blink_stones_list(input: List[int]) -> List[int]:
    next = []
    for value in input:
        if value == 0:
            next.append(1)
        elif len(str(value)) % 2 == 0:
            next.append(int(str(value)[: len(str(value)) // 2]))
            next.append(int(str(value)[len(str(value)) // 2 :]))
        else:
            next.append(value * 2024)
    return next


def part_one(input: List[int]) -> int:
    inputs = {i: [val] for i, val in enumerate(input)}
    for _ in range(25):
        for i in inputs:
            inputs[i] = blink_stones_list(inputs[i])
    return sum(len(val) for val in inputs.values())


def part_two(input: List[int]) -> int:
    stone_map = defaultdict(int)
    for stone in input:
        stone_map[stone] += 1

    for _ in range(75):
        new_map = defaultdict(int)
        for val, count in stone_map.items():
            if val == 0:
                new_map[1] += count
            elif len(str(val)) % 2 == 0:
                new_map[int(str(val)[: len(str(val)) // 2])] += count
                new_map[int(str(val)[len(str(val)) // 2 :])] += count
            else:
                new_map[val * 2024] += count
        stone_map = new_map
    return sum(stone_map.values())


if __name__ == "__main__":
    input = get_input()
    print(f"Part 1: {part_one(input)}")
    print(f"Part 2: {part_two(input)}")
