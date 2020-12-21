from collections import defaultdict
from os.path import abspath, dirname
from typing import Dict, List, Set, Tuple

INPUT_FILE = dirname(abspath(__file__)) + "/inputs/21-1.txt"

Parsed_Input = List[Tuple[List[str], List[str]]]

def get_input() -> Parsed_Input:
    contents = [
        tuple(
            token.replace(")", "").replace(",", "").split(" ")
            for token in line.split(" (contains ")
        )
        for line in open(INPUT_FILE).read().split("\n")
        if line != ""
    ]
    return contents


def build_allergen_dict(input: Parsed_Input) -> Dict[str, Set[str]]:
    """Builds a dictionary of allergens off an ingredient list"""
    allergen_dict = defaultdict(set)
    for line in input:
        ingredients, allergens = line
        for agent in allergens:
            if agent not in allergen_dict:
                allergen_dict[agent].update(ingredients)
            else:
                allergen_dict[agent].intersection_update(ingredients)

    # filter out any known duplicated ingredients from neighboring lists
    items_to_filter = [key for key, values in allergen_dict.items() if len(values) == 1]
    visited = set()
    while len(items_to_filter) != 0:
        filter_item = items_to_filter.pop()
        visited.add(filter_item)
        for agent in allergen_dict.keys():
            if agent == filter_item:
                continue
            allergen_dict[agent].difference_update(allergen_dict[filter_item])
            if len(allergen_dict[agent]) == 1 and agent not in visited:
                items_to_filter.append(agent)

    return allergen_dict


if __name__ == "__main__":
    input = get_input()
    allergen_dict = build_allergen_dict(input)

    non_reagents = set(
        item
        for line in input
        for item in line[0]
        if all(item not in agent_options for agent_options in allergen_dict.values())
    )
    count_non_reagents = sum(len(non_reagents.intersection(line[0])) for line in input)
    print(f"non-reagent count: {count_non_reagents}")
    allergen_list = sorted(
        [(allergen, ingredients.pop()) for allergen, ingredients in allergen_dict.items()],
        key=lambda x: x[0],
    )
    print(",".join(allergen[1] for allergen in allergen_list))
