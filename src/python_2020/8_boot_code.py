from os.path import abspath, dirname
from typing import List, Tuple

INPUT_FILE = dirname(abspath(__file__)) + "/inputs/8-1.txt"


def get_input():
    contents = open(INPUT_FILE).read().split("\n")
    return [[*line.split(" "), 0] for line in contents if line != ""]


jmp_nop = ["jmp", "nop"]


def find_loop(input: List[List], jmp_nop_to_change: int = 0) -> Tuple[int, int]:
    acc, index, jmp_nop_count = 0, 0, 0
    while input[index][2] != 1:
        cmd, cmd_num = input[index][0], int(input[index][1])

        input[index][2] = 1
        if cmd == "acc":
            acc += cmd_num
        elif cmd in jmp_nop:
            jmp_nop_count += 1
            if jmp_nop_count == jmp_nop_to_change:
                cmd = jmp_nop[jmp_nop.index(cmd) - 1]

        index += cmd_num if cmd == "jmp" else 1
        if index == len(input) - 1:
            break
        if index >= len(input):
            return 0, 0
    return acc, index


if __name__ == "__main__":
    input = get_input()
    index_to_change = 0
    while find_loop(get_input(), index_to_change)[1] != len(input) - 1:
        print(f"checked {index_to_change}")
        index_to_change += 1
        if index_to_change > len(input):
            break
    print(find_loop(input, index_to_change))
