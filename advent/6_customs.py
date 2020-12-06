from os.path import abspath, dirname

INPUT_FILE = dirname(abspath(__file__)) + "/inputs/6-1.txt"


def get_input():
    contents = open(INPUT_FILE).read().split("\n\n")
    group_answers = [
        [{answer for answer in individual} for individual in group.split("\n") if individual != ""]
        for group in contents
    ]
    return group_answers


if __name__ == "__main__":
    group_answers = get_input()
    group_union_counts = [len(set.union(*answers)) for answers in group_answers]
    print(f"union sum: {sum(group_union_counts)}")
    group_intersection_counts = [len(set.intersection(*answers)) for answers in group_answers]
    print(f"intersection sum: {sum(group_intersection_counts)}")
