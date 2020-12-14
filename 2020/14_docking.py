from copy import deepcopy
from collections import defaultdict
from re import match
from os.path import abspath, dirname
from typing import List

INPUT_FILE = dirname(abspath(__file__)) + "/inputs/14-1.txt"

def get_input() -> List:
    contents = [l.split(" = ") for l in open(INPUT_FILE).read().split("\n") if l != ""]
    return contents


memory = defaultdict(int)
entry_len = 36

def write_masked_values_to_memory(input):
    mask = ''
    for arg, value in input:
        if arg == 'mask':
            mask = value
        else:
            index = match('mem\[([0-9]+)\]', arg).group(1)
            masked_value = apply_basic_mask(int(value), mask)
            memory[index] = masked_value

def write_masked_addresses_to_memory(input):
    mask = ''
    for arg, value in input:
        if arg == 'mask':
            mask = value
        else:
            index = match('mem\[([0-9]+)\]', arg).group(1)
            addresses = apply_floating_mask(int(index), mask)
            for address in addresses:
                memory[address] = int(value)


def decimal_to_36_bit(decimal: int) -> List[str]:
    """Translates a decimal integer into a 36 bit binary array, stripping the leading 0b"""
    binary_num = list(bin(decimal))[2:]
    return ["0"] * (entry_len - len(binary_num)) + binary_num


def apply_basic_mask(decimal_num: int, mask: str) -> int:
    """Applies a 36-bit bitmask to a decimal integer.

    Bitmask is a string with X, 0, and 1 entries
    X is ignored while 0 and 1 entries overwrite the bit at that position
    """
    binary = decimal_to_36_bit(decimal_num)
    for i in range(len(mask)):
        if mask[i] != "X":
            binary[i] = mask[i]
    return int("0b" + "".join(binary), 2)

def apply_floating_mask(decimal_num: int, mask: str) -> List[int]:
    """Applies a 36-bit bitmask to a decimal integer.

    Bitmask is a string with X, 0, and 1 entries
    0 is ignored and 1 overwrites the bit at that position
    X splits the input into two numbers, one with 0 and the other with 1 at that position
    """
    binary = [decimal_to_36_bit(decimal_num)]
    for i in range(len(mask)):
        if mask[i] == "1":
            for entry in binary:
                entry[i] = mask[i]
        if mask[i] == "X":
            copied = deepcopy(binary)
            for j in range(len(copied)):
                copied[j][i] = '0'
                binary[j][i] = '1'
            binary.extend(copied)
    return [int("0b" + "".join(bit_array), 2) for bit_array in binary]

if __name__ == "__main__":
    input = get_input()
    # write_masked_values_to_memory(input)
    write_masked_addresses_to_memory(input)
    print(sum(memory.values()))
