from re import match
from os.path import abspath, dirname

INPUT_FILE = dirname(abspath(__file__)) + "/inputs/7-1.txt"


def get_input():
    contents = open(INPUT_FILE).read().split("\n")
    return [line.replace(".", "") for line in contents if line != ""]


def parse_line(line: str):
    [color, rule] = line.split(" bags contain ")
    luggage = Luggage_Tree.get_luggage_tree(color)
    if rule == "no other bags":
        return
    for bag_type in rule.split(", "):
        [bag_count, bag_color] = match("([0-9]) ([a-z ]+) bags?", bag_type).groups()
        luggage.add_child(Luggage_Tree.get_luggage_tree(bag_color), int(bag_count))


class Luggage_Tree:
    direct_access = dict()

    def __init__(self, color: str):
        self.color = color
        self.children = []
        self.parents = []
        Luggage_Tree.direct_access[color] = self

    def add_child(self, child_tree, child_count: int):
        child_tree.parents.append(self)
        self.children.append((child_tree, child_count))

    def count_containing_colors(self):
        """Counts how many other bags can hold `self`"""
        parents = [parent for parent in self.parents]
        containing_colors = set()
        while len(parents) != 0:
            next_parent = parents.pop(0)
            containing_colors.add(next_parent.color)
            parents.extend(
                [parent for parent in next_parent.parents if parent.color not in containing_colors]
            )
        return len(containing_colors)

    def count_required_bags(self):
        """counts how many bags are required to carry `self`"""
        if len(self.children) == 0:
            return 0
        return sum(
            [
                child_count + child_count * child.count_required_bags()
                for (child, child_count) in self.children
            ]
        )

    @staticmethod
    def get_luggage_tree(color: str):
        if color in Luggage_Tree.direct_access:
            return Luggage_Tree.direct_access[color]
        return Luggage_Tree(color)


if __name__ == "__main__":
    bag_color = "shiny gold"
    for line in get_input():
        parse_line(line)

    shiny_gold_bag = Luggage_Tree.get_luggage_tree("shiny gold")

    print(f"num options: {shiny_gold_bag.count_containing_colors()}")
    print(f"num required bags: {shiny_gold_bag.count_required_bags()}")
