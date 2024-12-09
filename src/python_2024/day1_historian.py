from collections import defaultdict
from os.path import abspath, dirname
from typing import List

INPUT_FILE = dirname(dirname(dirname(abspath(__file__)))) + "/input/2024/1-1.txt"


def get_input() -> List:
    contents = open(INPUT_FILE).read().split("\n")
    if contents[-1] == "":
        contents.pop()
    return contents


def partOne(input: List):
    sum = 0
    (freqL, freqR) = get_freqs(input)
    keysL, keysR = sorted(freqL.keys()), sorted(freqR.keys())
    indexL, indexR = 0, 0
    while indexL < len(keysL) and indexR < len(keysR):
        sum += abs(int(keysL[indexL]) - int(keysR[indexR]))

        freqR[keysR[indexR]] -= 1
        freqL[keysL[indexL]] -= 1
        if freqL[keysL[indexL]] == 0:
            indexL += 1
        if freqR[keysR[indexR]] == 0:
            indexR += 1
    return sum


def get_freqs(input: List):
    freqL = defaultdict(int)
    freqR = defaultdict(int)
    # default dict to store the count of each number
    # then iterate over dicts w/ sorted keys
    for line in input:
        if line == "":
            continue
        [l, r] = line.split()
        freqL[l] += 1
        freqR[r] += 1
    return (freqL, freqR)


def partTwo(input: List):
    sum = 0
    (freqL, freqR) = get_freqs(input)
    for line in input:
        [l, _] = line.split()
        sum += int(l) * freqR[l]
    return sum


if __name__ == "__main__":
    input = get_input()
    print(f"Part One: {partOne(input)}")
    print(f"Part Two: {partTwo(input)}")
