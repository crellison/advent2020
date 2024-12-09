from copy import deepcopy
from collections import defaultdict
from os.path import abspath, dirname
from typing import List, Tuple

INPUT_FILE = abspath(dirname(abspath(__file__)) + "/../../input/2024/5-test.txt")


def get_input() -> Tuple["Dag", List]:
    [edges, paths] = open(INPUT_FILE).read().split("\n\n")
    dag = Dag()
    for line in edges.split("\n"):
        start, end = map(int, line.split("|"))
        dag.add_edge(start, end)
    return dag, paths.split("\n")


class Node:
    def __init__(self, value: int):
        self.value = value
        self.deps = set()

    def add_dep(self, node: "Node"):
        self.deps.add(node.value)


class Dag:
    def __init__(self):
        self.nodes = {}

    def __add_node(self, value: int):
        if value not in self.nodes:
            self.nodes[value] = Node(value)

    def add_edge(self, src: int, dst: int):
        if src not in self.nodes:
            self.__add_node(src)
        if dst not in self.nodes:
            self.__add_node(dst)

        if src in self.nodes[dst].deps:
            return True

        if self.__has_invalid_cycle(src, dst):
            print(f"Cycle detected: {src} -> {dst}")
            return False

        self.nodes[src].add_dep(self.nodes[dst])
        return True

    def __has_invalid_cycle(self, src: int, dst: int):
        if src not in self.nodes or dst not in self.nodes:
            raise ValueError(f"Node {src} or {dst} does not exist")

        seen = set()
        nodes_to_check = [dst]
        while nodes_to_check:
            node = nodes_to_check.pop()
            if node == src:
                return True
            if node in seen:
                continue  # already handled
            seen.add(node)
            nodes_to_check.extend(self.nodes[node].deps)
        return False


def part_one(dag: Dag, paths: List) -> int:
    print("\n".join(f"{n.value} <- {n.deps}" for n in dag.nodes.values()))
    return 0


def part_two(paths: List) -> int:
    return 0


if __name__ == "__main__":
    dag, paths = get_input()
    print(f"Part 1: {part_one(dag, paths)}")
    print(f"Part 2: {part_two(paths)}")
