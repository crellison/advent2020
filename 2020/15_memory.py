from collections import defaultdict
from os.path import abspath, dirname
from typing import List

INPUT_FILE = dirname(abspath(__file__)) + "/inputs/15-1.txt"

def get_input() -> List:
    contents = [int(num) for num in open(INPUT_FILE).readline().split(",")]
    return contents

def elf_game(input: List[int], end_index: int = 2020) -> int:
    input_len = len(input)
    game_state = defaultdict(list)
    for i in range(input_len):
        game_state[input[i]].append(i)
    last = input[-1]
    for i in range(input_len, end_index):
        if len(game_state[last]) == 1:
            last = 0
        else:
            last = game_state[last][-1] - game_state[last][-2]
        game_state[last].append(i)
    return last


if __name__ == "__main__":
    input = get_input()
    print(elf_game(input))
    print(elf_game(input, end_index=30000000))
