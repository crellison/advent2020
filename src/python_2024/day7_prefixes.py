from os.path import abspath, dirname
from typing import List, Tuple

INPUT_FILE = abspath(dirname(abspath(__file__)) + "/../../input/2024/7-1.txt")


def parse_line(line: str) -> Tuple[int, List[int]]:
    """Lines are of form `target_num: num1 num2 ...`"""
    target_num, nums = line.split(": ")
    return int(target_num), [int(num) for num in nums.split(" ")]


def get_input() -> List[Tuple[int, List[int]]]:
    contents = open(INPUT_FILE).read().split("\n")
    if contents[-1] == "":
        contents.pop()
    return [parse_line(line) for line in contents]


def is_valid_expression(target_num: int, nums: List[int]) -> bool:
    if len(nums) == 1:
        return target_num == nums[0]
    if nums[0] > target_num:
        return False
    sum_nums = [nums[0] + nums[1], *nums[2:]]
    mult_nums = [nums[0] * nums[1], *nums[2:]]

    return is_valid_expression(target_num, sum_nums) or is_valid_expression(
        target_num, mult_nums
    )


def part_one(input: List[Tuple[int, List[int]]]) -> int:
    return sum(
        target_num
        for target_num, nums in input
        if is_valid_expression(target_num, nums)
    )


def is_valid_expression_with_concat(target_num: int, nums: List[int]) -> bool:
    if len(nums) == 1:
        return target_num == nums[0]
    if nums[0] > target_num:
        return False
    sum_nums = [nums[0] + nums[1], *nums[2:]]
    mult_nums = [nums[0] * nums[1], *nums[2:]]
    concat_nums = [int(f"{nums[0]}{nums[1]}"), *nums[2:]]

    return (
        is_valid_expression_with_concat(target_num, sum_nums)
        or is_valid_expression_with_concat(target_num, mult_nums)
        or is_valid_expression_with_concat(target_num, concat_nums)
    )


def part_two(input: List[Tuple[int, List[int]]]) -> int:
    return sum(
        target_num
        for target_num, nums in input
        if is_valid_expression_with_concat(target_num, nums)
    )


if __name__ == "__main__":
    input = get_input()
    print(f"Part 1: {part_one(input)}")
    print(f"Part 2: {part_two(input)}")
