from copy import deepcopy
from collections import defaultdict
from os.path import abspath, dirname
from typing import List, Tuple

INPUT_FILE = abspath(dirname(abspath(__file__)) + "/../../input/2024/19-1.txt")

"""
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb

make trie of valid patterns
recursively check if there exists a pattern that prefixes the input
    if not, then the input is not valid
if it exists, then loop recursively with the remaining input
"""


class PrefixTrie:
    def __init__(self, patterns: List[str]):
        self.root = {}
        for pattern in patterns:
            self.__add_pattern(pattern)

    def __add_pattern(self, pattern: str):
        node = self.root
        for i, char in enumerate(pattern):
            if char not in node:
                node[char] = {}
            node = node[char]
            if i == len(pattern) - 1:
                node["end"] = True

    def get_splits_on_prefix(self, word: str) -> List[Tuple[str, str]]:
        splits = []
        node = self.root
        for i, char in enumerate(word):
            if char not in node:
                return splits
            node = node[char]
            if "end" in node:
                splits.append((word[: i + 1], word[i + 1 :]))
        return splits


def get_input() -> Tuple[PrefixTrie, List[str]]:
    [patterns, words] = open(INPUT_FILE).read().split("\n\n")
    trie = PrefixTrie(patterns.split(", "))
    return trie, [w for w in words.split("\n") if w != ""]


def part_one(trie: PrefixTrie, words: List[str]) -> int:
    def is_valid(word: str) -> bool:
        if not word:
            return True
        splits = trie.get_splits_on_prefix(word)
        if not splits:
            return False
        return any(is_valid(suffix) for prefix, suffix in splits)

    return sum(1 for word in words if is_valid(word))


def part_two(trie: PrefixTrie, words: List[str]) -> int:
    ordering_memo = defaultdict(int)

    def valid_ordering_count(word: str) -> bool:
        if word in ordering_memo:
            return ordering_memo[word]
        if not word:
            return 1
        splits = trie.get_splits_on_prefix(word)
        if not splits:
            return 0
        ordering_memo[word] = sum(
            valid_ordering_count(suffix) for prefix, suffix in splits
        )
        return ordering_memo[word]

    return sum(valid_ordering_count(word) for word in words)


if __name__ == "__main__":
    trie, words = get_input()
    print(f"Part 1: {part_one(trie, words)}")
    print(f"Part 2: {part_two(trie, words)}")
