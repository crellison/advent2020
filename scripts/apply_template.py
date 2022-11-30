#!/usr/bin/python3

from os.path import dirname, abspath, basename, exists
from os import makedirs
from sys import argv
from glob import glob
from re import match

project_dir = dirname(dirname(abspath(__file__)))
language, ext = ".*/([a-z]+)\.tmpl.*", ".*\.tmpl\.([a-z]+)$"

templates = {
    match(language, file)[1]: (match(ext, file)[1], file)
    for file in glob(f"{project_dir}/templates/*.tmpl.*")
}


def apply_template(language: str, year: str, day: str, detail: str):
    """Creates a file for day and year with template"""
    (extension, template_path) = templates[language]
    template_string = open(template_path).read()
    templated_file = template_string.replace("${DAY}", day).replace("${YEAR}", year)

    inputs_dir = f"{project_dir}/input/{year}"
    makedirs(inputs_dir, exist_ok=True)
    out_file = f"src/{language}_{year}/day{day}_{detail}.{extension}"
    check_exists(out_file)

    open(f"{inputs_dir}/{day}-1.txt", mode="w").close()
    open(f"{inputs_dir}/{day}-test.txt", mode="w").close()
    with open(f"{project_dir}/{out_file}", mode="w") as file:
        file.write(templated_file)


def check_exists(out_file: str):
    """Checks if the target out_file already exists in filestructure"""
    if exists(f"{project_dir}/{out_file}"):
        print(f"File {out_file} already exists")
        overwrite = input("Overwrite file? [y/n]: ")
        if overwrite == "n":
            exit()
        elif overwrite != "y":
            print(f"Invalid response detected: {overwrite}. Terminating")
            exit()
        else:
            print(f"Overwriting file: {out_file}")


def parse_argv():
    [language, year, day, detail] = argv[1:]
    if language not in templates:
        raise Exception(
            f"Template for {language} not found in: {list(templates.keys())}"
        )

    return language, year, day, detail


if __name__ == "__main__":
    try:
        apply_template(*parse_argv())
    except Exception as e:
        print(e)
        print(f"\nUsage: $ python {basename(__file__)} language year day detail")
        exit()
