from collections import defaultdict
from os.path import abspath, dirname
from typing import Dict, List, Tuple
import re

INPUT_FILE = dirname(abspath(__file__)) + "/inputs/19-1.txt"


def get_input() -> Tuple[Dict, List]:
    [rules_raw, messages_raw] = open(INPUT_FILE).read().split("\n\n")
    rules = build_rules(rules_raw.split("\n"))
    messages = [m for m in messages_raw.split("\n") if m != ""]
    return rules, messages


def build_rules(rules_raw: List[str]):
    rules = dict()
    for rule in rules_raw:
        code, values = rule.split(": ")
        rules[code] = (
            values[1] if '"' in values else [option.split(" ") for option in values.split(" | ")]
        )
    return rules


def build_rule_regex(rules: defaultdict, code: str = "0"):
    if type(rules[code]) is str:
        return rules[code]
    else:
        rule = "|".join(
            f"({''.join([build_rule_regex(rules, rule_code) for rule_code in rule_option])})"
            for rule_option in rules[code]
        )
        return f"({rule})"


if __name__ == "__main__":
    rule_dict, messages = get_input()
    rule_regex = f"^{build_rule_regex(rule_dict)}$"
    rule_42, rule_31 = build_rule_regex(rule_dict, "42"), build_rule_regex(rule_dict, "31")
    rule_regex_42_31 = f"^(?P<rule_42>({rule_42})+)(?P<rule_31>({rule_31})+)$"

    sum_simple, sum_42_31 = 0, 0
    for message in messages:
        sum_simple += 1 if re.fullmatch(rule_regex, message) is not None else 0
        matches_42_31 = re.fullmatch(rule_regex_42_31, message)
        if matches_42_31 is not None:
            gp_42, gp_31 = matches_42_31.group("rule_42"), matches_42_31.group("rule_31")
            count_42, count_31 = len(re.sub(rule_42, "0", gp_42)), len(re.sub(rule_31, "0", gp_31))
            if count_42 > count_31:
                sum_42_31 += 1

    print(f"pt. 1: {sum_simple} valid of {len(messages)}")
    print(f"pt. 2: {sum_42_31} valid of {len(messages)}")
