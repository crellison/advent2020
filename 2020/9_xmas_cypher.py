from collections import defaultdict
from os.path import abspath, dirname
from typing import List, Tuple

INPUT_FILE = dirname(abspath(__file__)) + "/inputs/9-1.txt"


def get_input():
    contents = open(INPUT_FILE).read().split("\n")
    return [int(line) for line in contents if line != ""]


def init_valid_sums(preamble):
    valid_sums = defaultdict(list)
    for x in preamble:
        valid_sums[x] = defaultdict(int)
        for y in preamble:
            if y != x:
                valid_sums[x][y] = x + y
    return valid_sums


def find_first_invalid(input: List[int]) -> Tuple[int, int]:
    preamble = input[:25]
    valid_sums = init_valid_sums(preamble)
    for x in input[25:]:
        if not any([x in sums.values() for sums in valid_sums.values()]):
            return x, input.index(x)
        else:
            current_keys = list(valid_sums.keys())
            first_elt = current_keys.pop(0)
            valid_sums.pop(first_elt)
            valid_sums[x] = defaultdict(int)
            for key in current_keys:
                valid_sums[key].pop(first_elt)
                valid_sums[x][key] = x + key
    raise Exception("unable to find invalid value in cypher")

def find_contiguous_sum(input, target_sum):
    low, high, max_index = 0, 1, len(input) - 1
    current_sum = sum(input[low:high])
    while current_sum != target_sum:
        if current_sum > target_sum:
            low += 1
            high = low + 1
        else:
            high += 1
        if high > max_index:
            raise Exception(f"low and high collided at {high}")
        current_sum = sum(input[low:high])

    return input[low:high]

if __name__ == "__main__":
    input = get_input()
    x, index_x = find_first_invalid(input)
    print(x)
    contiguous_sum = find_contiguous_sum(input[:index_x], x)
    print(sum((min(contiguous_sum), max(contiguous_sum))))
