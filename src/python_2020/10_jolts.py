from os.path import abspath, dirname
from typing import Any, Callable, List
from collections import defaultdict

INPUT_FILE = dirname(abspath(__file__)) + "/inputs/10-1.txt"


def get_input() -> List[int]:
    contents = open(INPUT_FILE).read().split("\n")
    input = sorted([int(line) for line in contents if line != ""])
    return [0] + input + [input[-1] + 3]


def get_differences_and_options(input):
    differences = defaultdict(int)
    run_of_ones_count = 0
    options_count = 1
    for i in range(len(input) - 1):
        diff = input[i + 1] - input[i]
        differences[diff] += 1
        if diff == 1:
            run_of_ones_count += 1
        else:
            options_count *= count_options(run_of_ones_count)
            run_of_ones_count = 0
    return differences, options_count


def memoize(f: Callable[[Any], Any]) -> Callable[[Any], Any]:
    memo = {}

    def helper(x):
        if x not in memo:
            memo[x] = f(x)
        return memo[x]

    return helper


@memoize
def count_options(num):
    if num < 2:
        return 1
    else:
        max_range = 3 if num == 2 else 4
        return sum([count_options(num - x) for x in range(1, max_range)])


if __name__ == "__main__":
    input = get_input()
    differences, options = get_differences_and_options(input)
    print(differences[3] * differences[1])
    print(options)
