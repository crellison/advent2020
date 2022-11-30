import sys
from os.path import abspath, dirname
from typing import List, Set, Tuple

INPUT_FILE = dirname(abspath(__file__)) + "/inputs/22-1.txt"


def get_input() -> Tuple[List[int], List[int]]:
    p1, p2 = open(INPUT_FILE).read().split("\n\n")

    p1 = [int(card) for card in p1.split("\n")[1:] if card != ""]
    p2 = [int(card) for card in p2.split("\n")[1:] if card != ""]
    return p1, p2


def play_combat(p1: List[int], p2: List[int]) -> int:
    if len(p1) == 0 or len(p2) == 0:
        score = max(score_deck(p1), score_deck(p2))
        return score

    cards = p1.pop(0), p2.pop(0)
    winner = cards.index(max(cards))
    loser = (winner + 1) % 2
    spoils = [cards[winner], cards[loser]]
    if winner == 0:
        p1.extend(spoils)
    else:
        p2.extend(spoils)
    return play_combat(p1, p2)


def play_recursive_combat(p1: List[int], p2: List[int], past_games: Set = set()) -> int:
    if len(p1) == 0 or len(p2) == 0:
        winner = 0 if len(p1) != 0 else 1
        winning_deck = p1 if len(p1) != 0 else p2
        return winner, winning_deck

    game_state = hash(str(p1) + str(p2))
    if game_state in past_games:
        return 0, p1
    past_games.add(game_state)

    cards = p1.pop(0), p2.pop(0)
    winner = cards.index(max(cards))
    if len(p1) >= cards[0] and len(p2) >= cards[1]:
        winner, _ = play_recursive_combat(p1[: cards[0]], p2[: cards[1]], set())

    loser = (winner + 1) % 2
    spoils = [cards[winner], cards[loser]]
    if winner == 0:
        p1.extend(spoils)
    else:
        p2.extend(spoils)
    return play_recursive_combat(p1, p2, past_games)


def score_deck(deck: List[int]):
    deck_len = len(deck)
    score = sum((deck_len - i) * card for i, card in enumerate(deck))
    return score


if __name__ == "__main__":
    simple_score = play_combat(*get_input())
    print(f"combat score: {simple_score}")
    sys.setrecursionlimit(7500)  # oh god no
    winner, deck = play_recursive_combat(*get_input())
    sys.setrecursionlimit(1000)
    print(f"recursive combat score: {score_deck(deck)}")
