from re import match
from os.path import abspath, dirname
from typing import List

INPUT_FILE = dirname(abspath(__file__)) + "/inputs/18-1.txt"

def get_input() -> List:
    contents = [l for l in open(INPUT_FILE).read().split("\n") if l != '']
    return contents

first_elt_and_op = "([0-9]+) ([+*]) (.*)"

def eval_line_l2r(line: str):
    """Evaluates an expression with +* from left to right with no operator precedence.
    
    takes reversed lines.
    """
    if '*' not in line and '+' not in line:
        return int(line)
    if line.startswith(')'):
        paren_start = get_paren_start(line)
        paren_val = eval_line_l2r(line[1:paren_start])
        if paren_start == len(line) - 1:
            return paren_val    
        return eval_line_l2r(str(paren_val)+ line[paren_start+1:])
    
    matches = match(first_elt_and_op, line)
    if not matches:
        raise Exception(f'unable to parse line: {line}')
    
    num, op, rest_of_line = matches.groups()
    if op == '*':
        return eval_line_l2r(rest_of_line.strip()) * int(num)
    
    return eval_line_l2r(rest_of_line.strip()) + int(num)


def eval_line_sum_first(line: str):
    """Evaluates an expression with +* with addition operator precedence."""
    if '*' not in line and '+' not in line:
        return int(line)
    paren_index = line.find('(')
    if paren_index != -1:
        paren_end = get_paren_end(line, paren_index)

        new_line = str(eval_line_sum_first(line[paren_index+1:paren_end]))

        if paren_end != len(line) - 1:
            new_line = new_line + line[paren_end+1:]
        if paren_index != 0:
            new_line = line[:paren_index-1] + new_line
    
        return eval_line_sum_first(new_line)
    
    mult_inex = line.find('*')
    sum_index = line.find('+')
    if mult_inex != -1:
        return eval_line_sum_first(line[:mult_inex-1]) * eval_line_sum_first(line[mult_inex+1:])

    return eval_line_sum_first(line[:sum_index-1]) + eval_line_sum_first(line[sum_index+1:])

def get_paren_start(line: str):
    open_index = line.find("(")
    while line.count(')', 0, open_index) != line.count('(', 0, open_index+1):
        open_index = line.find('(', open_index +1)
    return open_index

def get_paren_end(line: str, start_index: int = 0):
    close_index = line.find(")")
    while line.count('(', start_index, close_index) != line.count(')', start_index, close_index+1):
        close_index = line.find(')', close_index +1)
    return close_index



if __name__ == "__main__":
    input = get_input()
    print(sum(eval_line_l2r(l[::-1]) for l in input))
    print(sum(eval_line_sum_first(l) for l in input))
