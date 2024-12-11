from functools import cmp_to_key
from os.path import abspath, dirname
from typing import List, Tuple

INPUT_FILE = abspath(dirname(abspath(__file__)) + "/../../input/2024/5-1.txt")


def get_input() -> Tuple["Graph", List]:
    [edges, paths] = open(INPUT_FILE).read().split("\n\n")
    dag = Graph()
    for line in edges.split("\n"):
        start, end = map(int, line.split("|"))
        dag.add_edge(start, end)
    return dag, [list(map(int, p.split(","))) for p in paths.split("\n") if p != ""]


class Node:
    def __init__(self, value: int):
        self.value = value
        self.deps = set()

    def add_dep(self, node: "Node"):
        self.deps.add(node.value)


class Graph:
    def __init__(self, is_acyclic=False):
        self.nodes = {}
        self.is_acyclic = is_acyclic

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

        # this makes it adhere to the DAG constraint
        if self.is_acyclic and self.__has_invalid_cycle(src, dst):
            return False

        self.nodes[src].add_dep(self.nodes[dst])
        return True
    
    def has_edge(self, src: int, dst: int) -> bool:
        return dst in self.nodes[src].deps

    def __has_invalid_cycle(self, src: int, dst: int):
        if src not in self.nodes or dst not in self.nodes:
            raise ValueError(f"Node {src} or {dst} does not exist")

        seen = set()
        nodes_to_check = [(dst, [dst])]
        while nodes_to_check:
            node, steps = nodes_to_check.pop()
            if node == src:
                print(f"Cycle detected: {' -> '.join(map(str, steps))}")
                return True
            if node in seen:
                continue  # already handled
            seen.add(node)
            nodes_to_check.extend((d, [*steps, d]) for d in self.nodes[node].deps)
        return False
    
    def is_valid_ordering(self, path: List[int]) -> bool:
        seen = set()
        for node_id in path:
            if node_id not in self.nodes:
                raise ValueError(f"Node {node_id} does not exist")
            if any(dep in seen for dep in self.nodes[node_id].deps):
                return False
            seen.add(node_id)
        return True
    
    def order_dag_nodes(self) -> List[int]:
        if not self.is_acyclic:
            raise ValueError("Graph is not acyclic. Cannot order nodes.")
        seen = set()
        ordered_nodes = []
        def visit_node(node_id: int):
            if node_id in seen:
                return
            seen.add(node_id)
            for dep in self.nodes[node_id].deps:
                visit_node(dep)
            ordered_nodes.append(node_id)

        for node in self.nodes.values():
            if node.value in seen:
                continue
            visit_node(node.value)
        return ordered_nodes


def part_one(dag: Graph, paths: List) -> int:
    mid_path_sum = 0
    for path in paths:
        if dag.is_valid_ordering(path):
            mid_path_sum += path[len(path) // 2]
    return mid_path_sum


def part_two(dag: Graph, paths: List) -> int:
    mid_path_sum = 0
    for path in paths:
        if dag.is_valid_ordering(path):
            continue
        sorted_path = sorted(path, key=cmp_to_key(lambda x, y: -1 if dag.has_edge(x, y) else 1 if dag.has_edge(y, x) else 0))
        mid_path_sum += sorted_path[len(sorted_path) // 2]
    return mid_path_sum


if __name__ == "__main__":
    dag, paths = get_input()
    # print("\n".join(f"{n.value} <- {n.deps}" for n in dag.nodes.values()))
    print(f"Part 1: {part_one(dag, paths)}")
    print(f"Part 2: {part_two(dag, paths)}")
