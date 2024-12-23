from copy import deepcopy
from collections import defaultdict
from os.path import abspath, dirname
from typing import List, Set, Iterable
from itertools import combinations

INPUT_FILE = abspath(dirname(abspath(__file__)) + "/../../input/2024/23-1.txt")


class Graph:
    def __init__(self, input: List):
        self.edges = defaultdict(list)
        self.nodes = set()
        self.__parse_input(input)

    def __parse_input(self, input: List):
        for line in input:
            a, b = line.split("-")
            self.edges[a].append(b)
            self.edges[b].append(a)
            self.nodes.add(a)
            self.nodes.add(b)

    def get_groups_for_node(self, node: str, size: int = 3) -> Set[Set[str]]:
        groups = set()
        neighbors = self.edges[node]
        for combo in combinations(neighbors, size - 1):
            if self.is_connected(combo):
                groups.add(frozenset([*combo, node]))
        return groups

    def is_connected(self, group: Iterable[str]) -> bool:
        for a, b in combinations(group, 2):
            if a not in self.edges[b]:
                return False
        return True

    def largest_group_for_node(self, node: str) -> Set[str]:
        neighbors = self.edges[node]
        for size in range(len(neighbors) + 1, 3, -1):
            next_groups = self.get_groups_for_node(node, size)
            if len(next_groups) > 0:
                if len(next_groups) != 1:
                    return set()
                return next(iter(next_groups))
        return set()


def get_input() -> Graph:
    contents = open(INPUT_FILE).read().split("\n")
    if contents[-1] == "":
        contents.pop()
    return Graph(contents)


def part_one(input: Graph) -> int:
    lan_groups = set()
    for node in input.nodes:
        if node.startswith("t"):
            lan_groups.update(input.get_groups_for_node(node))
    return len(lan_groups)


def part_two(input: Graph) -> int:
    largest_group = max(
        (input.largest_group_for_node(node) for node in input.nodes),
        key=lambda x: len(x),
    )
    return ",".join(sorted(largest_group))


if __name__ == "__main__":
    input = get_input()
    print(f"Part 1: {part_one(input)}")
    print(f"Part 2: {part_two(input)}")
