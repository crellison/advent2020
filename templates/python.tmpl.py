from copy import deepcopy
from collections import defaultdict
from os.path import abspath, dirname
from typing import List

INPUT_FILE = dirname(abspath(__file__)) + "/inputs/${DAY}-test.txt"

def get_input() -> List:
    contents = open(INPUT_FILE).read().split("\n")
    return contents

if __name__ == "__main__":
    input = get_input()
