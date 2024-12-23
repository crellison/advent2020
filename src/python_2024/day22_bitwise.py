from copy import deepcopy
from collections import defaultdict
from os.path import abspath, dirname
from typing import List
from functools import cache

INPUT_FILE = abspath(dirname(abspath(__file__)) + "/../../input/2024/22-1.txt")


def get_input() -> List:
    contents = open(INPUT_FILE).read().split("\n")
    if contents[-1] == "":
        contents.pop()
    return list(map(int, contents))


@cache
def nth_monkey_price(price: int, n: int) -> int:
    ret = price
    for _ in range(n):
        ret = (ret ^ (ret << 6)) & 0xFFFFFF
        ret = (ret ^ (ret >> 5)) & 0xFFFFFF
        ret = (ret ^ (ret << 11)) & 0xFFFFFF
    return ret


def part_one(input: List) -> int:
    return sum(nth_monkey_price(price, 2000) for price in input)


def part_two(input: List) -> int:
    change_costs = defaultdict(int)
    for price in input:
        seen = set()
        costs = []
        cost_deltas = []
        last_num = price
        for i in range(2000):
            costs.append(last_num % 10)
            last_num = nth_monkey_price(last_num, 1)
            if i != 0:
                cost_deltas.append(costs[-1] - costs[-2])
            else:
                cost_deltas.append(None)

        for i in range(4, 2000):
            last_four_chages = f"{cost_deltas[i-3:i+1]}"
            if last_four_chages in seen:
                continue
            seen.add(last_four_chages)
            change_costs[last_four_chages] += costs[i]

    return max(change_costs.values())


if __name__ == "__main__":
    input = get_input()
    print(f"Part 1: {part_one(input)}")
    print(f"Part 2: {part_two(input)}")
