from re import match
from collections import defaultdict
from os.path import abspath, dirname
from typing import List, Tuple
from functools import reduce

INPUT_FILE = dirname(abspath(__file__)) + "/inputs/16-1.txt"


def get_input() -> Tuple[defaultdict, List[int], List[List[int]]]:
    ticket_rules = defaultdict(set)
    [ticket_rules_raw, my_ticket_raw, other_tickets_raw] = open(INPUT_FILE).read().split("\n\n")

    for rule in ticket_rules_raw.split("\n"):
        matches = match("([a-z ]+): ([0-9\-]+) or ([0-9\-]+)", rule)
        if not matches:
            print(f"failed to parse rule: {rule}")
            continue
        name, *groups = matches.groups()

        for pair in groups:
            [min, max] = pair.split("-")
            ticket_rules[name].update(range(int(min), int(max) + 1))

    my_ticket = [int(i) for i in my_ticket_raw.split("\n")[1].split(",")]

    other_tickets = [
        [int(i) for i in ticket.split(",")]
        for ticket in other_tickets_raw.split("\n")[1:]
        if ticket != ""
    ]

    return ticket_rules, my_ticket, other_tickets


def scanning_error_rate(ticket_rules: defaultdict, tickets: List[List[int]]) -> List[List[int]]:
    """Calculates the scanning error rate = the product of all values invalid for any field"""
    error_rate = sum(
        number
        for ticket in tickets
        for number in ticket
        if all(number not in field_range for field_range in ticket_rules.values())
    )
    return error_rate


def filter_invalid_tickets(ticket_rules: defaultdict, tickets: List[List[int]]) -> List[List[int]]:
    def ticket_filter(ticket: List[int]) -> bool:
        is_valid = all(
            any(number in field_range for field_range in ticket_rules.values()) for number in ticket
        )
        return is_valid

    return list(filter(ticket_filter, tickets))


def reduce_tickets(tickets: List[List[int]]) -> defaultdict:
    """Reduces a list of tickets into a list of sets with all seen ticket values at each index."""
    ticket_sets = defaultdict(set)
    for ticket in tickets:
        for i in range(len(ticket)):
            ticket_sets[i].add(ticket[i])
    return ticket_sets


def find_ticket_fields(tickets: defaultdict, ticket_rules: defaultdict):
    """Assigns ticket fields to indexes based on the ticket rules and seen ticket values."""
    index_field_map = defaultdict(str)
    unassigned_fields = set(ticket_rules.keys())
    while len(unassigned_fields) != 0:
        unassigned_tickets = [i for i in tickets if i not in index_field_map]
        for i in unassigned_tickets:
            possible_fields = [
                field for field in unassigned_fields if tickets[i].issubset(ticket_rules[field])
            ]
            if len(possible_fields) == 0:
                raise Exception(f"mistake in rule parsing: no possible values for {i}")
            if len(possible_fields) == 1:
                found = possible_fields[0]
                index_field_map[i] = found
                unassigned_fields.remove(found)
    return index_field_map


if __name__ == "__main__":
    ticket_rules, my_ticket, other_tickets = get_input()
    print(f"scanning error rate: {scanning_error_rate(ticket_rules, other_tickets)}")
    valid_tickets = filter_invalid_tickets(ticket_rules, other_tickets)
    ticket_sets = reduce_tickets(valid_tickets + [my_ticket])
    field_map = find_ticket_fields(ticket_sets, ticket_rules)
    departure_values = [
        my_ticket[key] for key, value in field_map.items() if value.startswith("departure")
    ]
    print(f"product of departure fields: {reduce(lambda x, y: x * y, departure_values)}")
