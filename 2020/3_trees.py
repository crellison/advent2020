from os.path import abspath, dirname
from functools import reduce

INPUT_FILE = dirname(abspath(__file__)) + "/inputs/3-1.txt"


def get_input():
    contents = open(INPUT_FILE).read()
    trees = contents.split("\n")
    return trees


def product(numbers: list):
    return reduce(lambda x, y: x * y, numbers)


def num_trees_hit(trees: list, vert_step: int, right_step: int) -> int:
    """Counts number of 'trees' hit when traversing a slope

    trees is a list with string lines of the format
    `...............##........##....`

    `#` represents a tree and `.` represents an open slope.
    The tree pattern repeats to the right.

    Starting at the upper left, this counts how many trees you encounter if you
    traverse down the treed slope.
    """
    position = [0, 0]
    tree_count = 0
    while position[0] < len(trees) - 1:
        if trees[position[0]][position[1]] == "#":
            tree_count += 1
        position[0] += vert_step
        position[1] = (len(trees[0]) + position[1] + right_step) % len(trees[0])
    print(f"hit {tree_count} trees with move {vert_step} vert and {right_step} across")
    return tree_count


if __name__ == "__main__":
    trees = get_input()
    tree_count_product = product(
        [
            num_trees_hit(trees, vert, right)
            for [right, vert] in [[1, 1], [3, 1], [5, 1], [7, 1], [1, 2]]
        ]
    )
    print(tree_count_product)
