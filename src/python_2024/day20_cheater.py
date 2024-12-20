from copy import deepcopy
from collections import defaultdict
from os.path import abspath, dirname
from typing import List, Tuple

INPUT_FILE = abspath(dirname(abspath(__file__)) + "/../../input/2024/20-1.txt")


class Racetrack:
    def __init__(self, contents: List[str]):
        self.start = None
        self.end = None
        self.map = defaultdict(int)
        for i, line in enumerate(contents):
            for j, char in enumerate(line):
                if char == "S":
                    self.map[(i, j)] = 0
                    self.start = (i, j)
                elif char == "E":
                    self.end = (i, j)
                    self.map[(i, j)] = len(line) * len(contents)
                elif char == "#":
                    self.map[(i, j)] = -1
                else:
                    self.map[(i, j)] = len(line) * len(contents)
        self.__calc_initial_distances()

    def __get_next(self, pos: Tuple[int, int]) -> List[Tuple[int, int]]:
        possible_nexts = [
            p
            for p in [
                (pos[0] + 1, pos[1]),
                (pos[0] - 1, pos[1]),
                (pos[0], pos[1] + 1),
                (pos[0], pos[1] - 1),
            ]
            if p in self.map and self.map[p] != -1 and self.map[p] > self.map[pos]
        ]
        if len(possible_nexts) != 1:
            raise ValueError(f"Multiple possible nexts: {possible_nexts}")
        return possible_nexts[0]

    def __calc_initial_distances(self) -> None:
        next = self.start
        while next != self.end:
            last_cost = self.map[next]
            next = self.__get_next(next)
            self.map[next] = last_cost + 1

    def get_locations_in_n_steps(
        self, pos: Tuple[int, int], n: int
    ) -> List[Tuple[Tuple[int, int], int]]:
        """gets locations within n steps of the start if they are further along the track"""
        locations = []
        for i in range(n + 1):
            considered = set()
            for j in range(i + 1):
                considered.add((pos[0] + (i - j), pos[1] + j))
                considered.add((pos[0] - (i - j), pos[1] - j))
                considered.add((pos[0] + (i - j), pos[1] - j))
                considered.add((pos[0] - (i - j), pos[1] + j))
            for loc in considered:
                if (
                    loc in self.map
                    and self.map[loc] != -1
                    and self.map[loc] > self.map[pos] + i
                ):
                    locations.append((loc, self.map[loc] - self.map[pos] - i))

        return locations

    def get_step_cheats(self, n: int) -> List[Tuple[str, int]]:
        cheats = []
        for pos in self.map:
            if self.map[pos] == -1:
                continue
            cheats.extend(self.get_locations_in_n_steps(pos, n))
        return cheats


def get_input() -> Racetrack:
    contents = open(INPUT_FILE).read().split("\n")
    track = Racetrack(contents)
    return track


def part_one(input: Racetrack) -> int:
    return sum(1 for _, cost in input.get_step_cheats(2) if cost >= 100)


def part_two(input: Racetrack) -> int:
    return sum(1 for _, cost in input.get_step_cheats(20) if cost >= 100)


if __name__ == "__main__":
    track = get_input()
    print(f"Part 1: {part_one(track)}")
    print(f"Part 2: {part_two(track)}")
