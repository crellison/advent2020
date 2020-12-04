from os.path import abspath, dirname, split
from re import match

INPUT_FILE = dirname(abspath(__file__)) + "/inputs/4-1.txt"

required_fields = set(["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"])
optional_fields = set(["cid"])

eye_colors = { "amb", "blu", "brn", "gry", "grn", "hzl", "oth" }

def not_btw(num: int, min_val: int, max_val: int):
    return num < min_val or num > max_val


def validate_passport(*, byr: str, iyr: str, eyr: str, hgt: str, hcl: str, ecl: str, pid: str, cid = ''):
    invalid_passport_exception = lambda val: Exception(f"Passport is invalid: {val}")
    [byr, iyr, eyr] = [int(val) for val in [byr, iyr, eyr]]
    if not_btw(byr, 1920, 2002):
        raise invalid_passport_exception(byr)
    if not_btw(iyr, 2010, 2020):
        raise invalid_passport_exception(iyr)
    if not_btw(eyr, 2020, 2030):
        raise invalid_passport_exception(eyr)
    
    height = int(hgt[:-2])
    if match("^[0-9]+(cm|in)$", hgt) is None:
        raise invalid_passport_exception(hgt)
    if hgt.endswith("in") and not_btw(height, 59, 76):
        raise invalid_passport_exception(hgt)
    if hgt.endswith("cm") and not_btw(height, 150, 193):
        raise invalid_passport_exception(hgt)

    if match("^#[a-f0-9]{6}$", hcl) is None:
        raise invalid_passport_exception(hcl)
    if match('^[0-9]{9}$', pid) is None:
        raise invalid_passport_exception(pid)
    if ecl not in eye_colors:
        raise invalid_passport_exception(ecl)
    return 1

def validate_passports():
    file = open(INPUT_FILE)
    current_passport = {}
    total_passports = 0
    valid_passport_count = 0
    while True:
        line = file.readline()
        if line == "\n" or line == "":
            total_passports += 1
            try:
                validate_passport(**current_passport)
                valid_passport_count += 1
            except Exception as e:
                pass

            current_passport = {}
            if line == "":
                break
        else:
            new_values = {
                key: value for [key, value] in [pair.split(":") for pair in line.replace('\n', '').split(" ")]
            }
            current_passport.update(new_values)
    print(f"found {valid_passport_count} of {total_passports} valid")
    return valid_passport_count


if __name__ == "__main__":
    validate_passports()
