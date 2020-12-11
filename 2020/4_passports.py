from os.path import abspath, dirname
from re import match

INPUT_FILE = dirname(abspath(__file__)) + "/inputs/4-1.txt"


def get_input():
    passports = [
        dict(item.split(":") for item in line.replace("\n", " ").strip().split(" "))
        for line in open(INPUT_FILE).read().split("\n\n")
        if line != ""
    ]
    return passports


required_fields = set(["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"])
optional_fields = set(["cid"])

eye_colors = {"amb", "blu", "brn", "gry", "grn", "hzl", "oth"}


def not_btw(num: int, min_val: int, max_val: int):
    return num < min_val or num > max_val


def validate_passport(*, byr="", iyr="", eyr="", hgt="", hcl="", ecl="", pid="", cid=""):
    try:
        [byr, iyr, eyr] = [int(val) for val in [byr, iyr, eyr]]
        if not_btw(byr, 1920, 2002):
            return 0
        if not_btw(iyr, 2010, 2020):
            return 0
        if not_btw(eyr, 2020, 2030):
            return 0

        height = int(hgt[:-2])
        if match("^[0-9]+(cm|in)$", hgt) is None:
            return 0
        if hgt.endswith("in") and not_btw(height, 59, 76):
            return 0
        if hgt.endswith("cm") and not_btw(height, 150, 193):
            return 0

        if match("^#[a-f0-9]{6}$", hcl) is None:
            return 0
        if match("^[0-9]{9}$", pid) is None:
            return 0
        if ecl not in eye_colors:
            return 0
    except Exception:
        return 0
    return 1


def validate_passports(passports):
    valid_passport_count = sum([validate_passport(**passport) for passport in passports])
    print(f"found {valid_passport_count} of {len(passports)} valid")
    return valid_passport_count


if __name__ == "__main__":
    passports = get_input()
    validate_passports(passports)
