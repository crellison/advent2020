from os.path import abspath, dirname

INPUT_FILE = dirname(abspath(__file__)) + "/inputs/2-1.txt"


def get_input():
    """retrieves and formats input into list

    File format example
    2-3 a: skksckaa

    File format description
    <int>-<int> <char>: <string>
    """
    contents = open(INPUT_FILE).read().split("\n")
    passwords_and_rules = []
    for line in contents:
        if not line:
            continue
        [rules, password] = line.split(": ")
        [num_1, num_2, char] = rules.replace("-", " ").split(" ")
        passwords_and_rules.append([int(num_1), int(num_2), char, password])
    return passwords_and_rules


def validate_password_counts(pw_and_rules):
    [min_count, max_count, char, password] = pw_and_rules
    counts_of_char = len(password.split(char)) - 1
    return counts_of_char >= min_count and counts_of_char <= max_count


def validate_password_indexes(pw_and_rules):
    [index_1, index_2, char, password] = pw_and_rules
    return (password[index_1 - 1] == char) ^ (password[index_2 - 1] == char)


def find_valid_passwords(passwords_and_rules):
    """finds invalid passwords given input"""
    valid_passwords = [pw for pw in filter(validate_password_indexes, passwords_and_rules)]
    return valid_passwords


if __name__ == "__main__":
    valid_pw = find_valid_passwords(get_input())
    print(len(valid_pw))
