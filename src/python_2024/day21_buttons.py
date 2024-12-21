from copy import deepcopy
from collections import defaultdict
from os.path import abspath, dirname
from typing import List, Tuple
from functools import cache

INPUT_FILE = abspath(dirname(abspath(__file__)) + "/../../input/2024/21-1.txt")

# cannot go over a location without a button (we don't actually need to worry about this ATM)
# must choose the shortest path from start to end (this will be deterministic by button location)
# complexity of sequence is number of moves * numeric part of code


class Keypad:
    def __init__(self, contents: List[List[str]]):
        self.locations = defaultdict(str)
        self.buttons = defaultdict(Tuple[int, int])
        for i, line in enumerate(contents):
            for j, char in enumerate(line):
                if char is not None:
                    self.locations[(i, j)] = char
                    self.buttons[char] = (i, j)

    @cache
    def get_path_cost(self, start: str, end: str) -> int:
        arrow_keypad = Keypad([[None, "^", "A"], ["<", "v", ">"]])
        di = arrow_keypad.buttons[end][0] - arrow_keypad.buttons[start][0]
        dj = arrow_keypad.buttons[end][1] - arrow_keypad.buttons[start][1]
        return abs(di) + abs(dj)

    @cache
    def get_paths_between(self, start: str, end: str) -> List[str]:
        if start == end:
            return ["A"]
        if start not in self.buttons or end not in self.buttons:
            raise ValueError(f"Start or end not in buttons: {start}, {end}")

        di, dj = (
            self.buttons[end][0] - self.buttons[start][0],
            self.buttons[end][1] - self.buttons[start][1],
        )

        possible_paths = []

        if abs(di) + abs(dj) == 1:
            possible_paths.append(
                ("<" if dj == -1 else ">" if dj == 1 else "^" if di == -1 else "v")
                + "A"
            )
            return possible_paths

        if di != 0:
            next_i = (self.buttons[start][0] + di / abs(di), self.buttons[start][1])

            if next_i in self.locations:
                possible_paths.extend(
                    f"{'v' if di > 0 else '^'}{seq}"
                    for seq in self.get_paths_between(self.locations[next_i], end)
                )
        if dj != 0:
            next_j = (self.buttons[start][0], self.buttons[start][1] + dj / abs(dj))
            if dj != 0 and next_j in self.locations:
                possible_paths.extend(
                    f"{'>' if dj > 0 else '<'}{seq}"
                    for seq in self.get_paths_between(self.locations[next_j], end)
                )

        return possible_paths

    @cache
    def get_steps_to_button(self, start: str, end: str) -> str:
        if start == end:
            return "A"
        if start not in self.buttons or end not in self.buttons:
            raise ValueError(f"Start or end not in buttons: {start}, {end}")

        possible_paths = self.get_paths_between(start, end)

        if len(possible_paths) == 1:
            return possible_paths[0]

        best_path = min(
            possible_paths,
            key=lambda x: sum(
                self.get_path_cost(x[i], x[i + 1]) for i in range(len(x) - 1)
            ),
        )

        return best_path

    @cache
    def get_button_sequence(self, buttons: str) -> str:
        seq = "".join(
            self.get_steps_to_button(buttons[i], buttons[i + 1])
            for i in range(len(buttons) - 1)
        )
        return seq


def get_input() -> List:
    contents = open(INPUT_FILE).read().split("\n")
    if contents[-1] == "":
        contents.pop()
    return contents


def part_one(input: List) -> int:
    num_keypad = Keypad(
        [["7", "8", "9"], ["4", "5", "6"], ["1", "2", "3"], [None, "0", "A"]]
    )
    arrow_keypad = Keypad([[None, "^", "A"], ["<", "v", ">"]])
    total_complexity = 0
    for item in input:
        last_path = num_keypad.get_button_sequence("A" + item)
        for iter in range(2):
            last_path = arrow_keypad.get_button_sequence("A" + last_path)
            print(last_path)
        total_complexity += len(last_path) * int(item[:-1])
    return total_complexity


def part_two(input: List) -> int:
    num_keypad = Keypad(
        [["7", "8", "9"], ["4", "5", "6"], ["1", "2", "3"], [None, "0", "A"]]
    )
    arrow_keypad = Keypad([[None, "^", "A"], ["<", "v", ">"]])

    @cache
    def get_min_path_cost(path: str, depth: int = 25) -> int:
        """
        Caching by depth is more correct than caching by path.
        This also reduces the amount of memory used, as we only need to retain cost
        """
        pad = num_keypad if depth == 25 else arrow_keypad
        cost = 0
        seq = "A" + path
        for i in range(len(seq) - 1):
            options = pad.get_paths_between(seq[i], seq[i + 1])
            if depth == 0:
                cost += min(len(option) for option in options)
            else:
                cost += min(get_min_path_cost(option, depth - 1) for option in options)
        return cost

    total_complexity = 0
    for item in input:
        total_complexity += get_min_path_cost(item) * int(item[:-1])
    return total_complexity


if __name__ == "__main__":
    input = get_input()
    print(f"Part 1: {part_one(input)}")
    print(f"Part 2: {part_two(input)}")
