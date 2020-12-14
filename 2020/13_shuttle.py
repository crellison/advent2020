from math import ceil, floor
from functools import reduce
from os.path import abspath, dirname
from typing import List, Tuple

INPUT_FILE = dirname(abspath(__file__)) + "/inputs/13-1.txt"


def get_input() -> List:
    input = open(INPUT_FILE)
    timestamp = int(input.readline())
    busses = [int(bus_id) if bus_id != "x" else bus_id for bus_id in input.readline().split(",")]
    return timestamp, busses


def get_first_bus(timestamp, busses):
    first_bus = min(
        [(bus_id, bus_id * ceil(timestamp / bus_id)) for bus_id in busses if bus_id != "x"],
        key=lambda entry: entry[1],
    )
    return first_bus


def bezout_coeff(a, b):
    """Given coprimes a,b: calculates x, y s.t. ax + by = 1."""
    old_remainder, remainder = a, b
    old_a_coeff, a_coeff = 1, 0
    old_b_coeff, b_coeff = 0, 1
    # find the coefficients
    while remainder != 0:
        quotient = old_remainder // remainder
        old_remainder, remainder = remainder, old_remainder - quotient * remainder
        [old_a_coeff, a_coeff] = [a_coeff, old_a_coeff - quotient * a_coeff]
        old_b_coeff, b_coeff = b_coeff, old_b_coeff - quotient * b_coeff
    # return the coefficients
    return old_a_coeff, old_b_coeff


def chinese_remainder_theorem(input: List[Tuple[int, int]]) -> int:
    """Calculates x s.t. x % num = remainder for remainder, num in input

    The solution is minimized at each step by the collective product of the
    inputs thus far.
    """

    def reducer(x: Tuple[int, int], y: Tuple[int, int]) -> Tuple[int, int]:
        x_mod, x_id = x
        y_mod, y_id = y
        x_bc, y_bc = bezout_coeff(x_id, y_id)
        new_mod = y_bc * y_id * x_mod + x_bc * x_id * y_mod
        current_product = y_id * x_id

        # minimize new modulus to smallest positive option
        if new_mod < 0:
            new_mod += ceil(abs(new_mod) / current_product) * current_product
        else:
            new_mod -= floor(abs(new_mod) / current_product) * current_product

        return (new_mod, y_id * x_id)

    x, _ = reduce(reducer, input)
    return x

if __name__ == "__main__":
    [timestamp, busses] = get_input()
    # bus_id, departure_time = get_first_bus(timestamp, busses)
    # print(bus_id, departure_time)
    busses_with_remainders = [
        ((busses[i] - i) % busses[i], busses[i]) for i in range(len(busses)) if busses[i] != "x"
    ]
    x = chinese_remainder_theorem(busses_with_remainders)
    print(x)
