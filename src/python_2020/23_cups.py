from typing import Dict, List

TEST_INPUT = [int(i) for i in "389125467"]
INPUT = [int(i) for i in "712643589"]
BIG_TEST_INPUT = TEST_INPUT + list(range(10, 1000000 + 1))
BIG_INPUT = INPUT + list(range(10, 1000000 + 1))


def get_input(raw_input: List[str]) -> Dict[int, int]:
    ring = {num: next_num for num, next_num in zip(raw_input, raw_input[1:] + [raw_input[0]])}
    return ring


def play_game(cups: Dict[int, int], starting_cup: int, rounds: int):
    cup_max = len(cups.keys())
    print(f"playing {cup_max} cup game for {rounds} rounds")

    def move_cups(current_cup: int):
        next_to_pickup = cups[current_cup]
        picked_up = []
        while len(picked_up) < 3:
            picked_up.append(next_to_pickup)
            next_to_pickup = cups.pop(next_to_pickup)

        cups[current_cup] = next_to_pickup

        destination_cup = current_cup - 1
        while destination_cup not in cups:
            destination_cup -= 1
            if destination_cup < 1:
                destination_cup = cup_max

        destination_next = cups[destination_cup]
        for key, value in zip([destination_cup] + picked_up, picked_up + [destination_next]):
            cups[key] = value

        next_cup = cups[current_cup]
        return next_cup

    next_cup = starting_cup
    for i in range(rounds):
        next_cup = move_cups(next_cup)


if __name__ == "__main__":
    little_cups = get_input(INPUT)
    play_game(little_cups, INPUT[0], 100)
    after_one = [little_cups[1]]
    for _ in range(len(INPUT) - 2):
        after_one.append(little_cups[after_one[-1]])
    seq_after_one = "".join(str(i) for i in after_one)
    print(f"sequence after 1: {seq_after_one}")

    big_cups = get_input(BIG_INPUT)
    play_game(big_cups, BIG_INPUT[0], 10000000)
    after_one = big_cups[1]
    after_after_one = big_cups[after_one]
    print(f"{after_one} * {after_after_one} = {after_one * after_after_one}")
