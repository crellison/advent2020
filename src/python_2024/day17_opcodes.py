from copy import deepcopy
from collections import defaultdict
from os.path import abspath, dirname
from typing import List
import re

INPUT_FILE = abspath(dirname(abspath(__file__)) + "/../../input/2024/17-1.txt")


def get_input() -> "Computer":
    [registers, program] = open(INPUT_FILE).read().split("\n\n")
    [A, B, C] = list(map(int, re.findall(r"[A-C]: (\d+)", registers)))
    program = list(map(int, re.findall(r"\d+", program)))

    return Computer(program, A, B, C)


"""
input is in format below:
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
"""


class Computer:
    def __init__(self, program: List[str], A: int = 0, B: int = 0, C: int = 0):
        self.program = program
        self.original_registers = {"A": A, "B": B, "C": C}
        self.registers = {"A": A, "B": B, "C": C}
        self.instruction_pointer = 0
        self.output = []

    def __str__(self):
        return f"Register A: {self.registers['A']}\nRegister B: {self.registers['B']}\nRegister C: {self.registers['C']}\nProgram: {self.program} at {self.instruction_pointer}\nOutput: {self.output}"

    def reset(self):
        self.registers = {key: val for key, val in self.original_registers.items()}
        self.instruction_pointer = 0
        self.output = []

    def run_program(self) -> str:
        while self.instruction_pointer < len(self.program):
            self.run_step()

        return ",".join(map(str, self.output))

    def run_to_next_output(self) -> None:
        start_length = len(self.output)
        while start_length == len(self.output):
            if self.instruction_pointer >= len(self.program):
                return
            self.run_step()

    def run_step(self) -> None:
        step_pointer = True
        if self.program[self.instruction_pointer] == 0:
            self.__avd()
        elif self.program[self.instruction_pointer] == 1:
            self.__bxl()
        elif self.program[self.instruction_pointer] == 2:
            self.__bst()
        elif self.program[self.instruction_pointer] == 3:
            step_pointer = self.__jnz()
        elif self.program[self.instruction_pointer] == 4:
            self.__bxc()
        elif self.program[self.instruction_pointer] == 5:
            self.__out()
        elif self.program[self.instruction_pointer] == 6:
            self.__bdv()
        elif self.program[self.instruction_pointer] == 7:
            self.__cdv()
        else:
            raise ValueError(
                f"Invalid opcode: {self.program[self.instruction_pointer]}"
            )

        if step_pointer:
            self.instruction_pointer += 2

    def __combo_operand(self) -> int:
        op_val = self.program[self.instruction_pointer + 1]
        if op_val in range(4):
            return op_val
        elif op_val == 4:
            return self.registers["A"]
        elif op_val == 5:
            return self.registers["B"]
        elif op_val == 6:
            return self.registers["C"]
        else:
            raise ValueError(f"Invalid combo operand: {op_val}")

    def __literal_operand(self) -> int:
        return self.program[self.instruction_pointer + 1]

    def __avd(self) -> None:
        self.registers["A"] = self.registers["A"] // (2 ** self.__combo_operand())

    def __bxl(self) -> None:
        self.registers["B"] = self.registers["B"] ^ self.__literal_operand()

    def __bst(self) -> None:
        self.registers["B"] = self.__combo_operand() & 7

    def __jnz(self) -> bool:
        if self.registers["A"] == 0:
            return True
        else:
            self.instruction_pointer = self.__literal_operand()
            return False

    def __bxc(self) -> None:
        self.registers["B"] = self.registers["B"] ^ self.registers["C"]

    def __out(self) -> None:
        self.output.append(self.__combo_operand() % 8)

    def __bdv(self) -> None:
        self.registers["B"] = self.registers["A"] // (2 ** self.__combo_operand())

    def __cdv(self) -> None:
        self.registers["C"] = self.registers["A"] // (2 ** self.__combo_operand())


def part_two(computer: Computer) -> int:
    original_program = computer.program

    # build output from the back forwards
    candidate = 0
    for i in range(len(original_program) - 1, -1, -1):
        candidate *= 8
        target_out = original_program[i:]
        while True:
            computer.reset()
            computer.registers["A"] = candidate
            computer.run_program()
            if computer.output == target_out:
                break
            candidate += 1

    return candidate


# def is_valid_a_register(a_register: int) -> bool:
#     computer.reset()
#         computer.registers["A"] = a_register
#         while computer.instruction_pointer < len(original_program):
#             computer.run_to_next_output()
#             if len(original_program) < len(computer.output):
#                 return False
#             if any(
#                 original_program[i] != computer.output[i]
#                 for i in range(len(computer.output))
#             ):
#                 return False
#             print(f"Checking {a_register}")
#             print(computer)
#         return original_program == computer.output

#     for i in range(0, 100):
#         if is_valid_a_register(i):
#             return i
#     return 0


if __name__ == "__main__":
    computer = get_input()
    print(f"Part 1: {computer.run_program()}")
    print(f"Part 2: {part_two(computer)}")
