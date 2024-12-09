from re import findall, finditer
from os.path import abspath, dirname

INPUT_FILE = abspath(dirname(abspath(__file__)) + "/../../input/2024/3-1.txt")


def get_input() -> str:
    return open(INPUT_FILE).read()


def part_one(input: str) -> int:
    return sum(int(x) * int(y) for x, y in findall(r"mul\((\d+),(\d+)\)", input))


def part_two(input: str) -> int:
    total = 0
    disabled_indices = [*(m.end() for m in finditer(r"don't\(\)", input)), len(input)]
    enabled_indices = [0, *(m.end() for m in finditer(r"do\(\)", input))]
    enabled_ranges = []
    d_i, e_i = 0, 0
    for _ in range(max(len(disabled_indices), len(enabled_indices))):
        d_i = next(
            (
                i
                for i in range(d_i, len(disabled_indices))
                if disabled_indices[i] > enabled_indices[e_i]
            ),
            -1,
        )
        if d_i == -1:
            break
        enabled_ranges.append((enabled_indices[e_i], disabled_indices[d_i]))
        e_i = next(
            (
                i
                for i in range(e_i + 1, len(enabled_indices))
                if enabled_indices[i] > disabled_indices[d_i]
            ),
            -1,
        )
        if e_i == -1:
            break
    print(enabled_ranges)
    total += sum(
        int(x) * int(y)
        for start, end in enabled_ranges
        for x, y in findall(r"mul\((\d+),(\d+)\)", input[start:end])
    )

    return total


if __name__ == "__main__":
    input = get_input()
    print(f"Part 1: {part_one(input)}")
    print(f"Part 2: {part_two(input)}")
