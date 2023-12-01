# Advent of Code Solutions

My (poorly written) solutions to the [Advent of Code](https://adventofcode.com/).
No claims about optimized solutions are being made here.

## Templating Days

Certain scripting languages are easy to use in an isolated manner without additional setup.
For these, I've found this little templater to be helpful.

Use `./scripts/apply_template.py` to generate solution templates by day.
Template files can be found in `./templates`.
Template files are names as follows: `<language>.tmpl.<language-extension>`

```sh
$ ./scripts/apply_template.py
not enough values to unpack (expected 4, got 0)

Usage: $ python apply_template.py language year day detail
```

## Specific instructions for languages

### Python

```sh
$ python /path/to/day_file.py
# output logged below
```

### Rust

> Disclaimer: When writing this code, I was rather unfamiliar with the Rust language.
> I probably still am unfamiliar with the language.

I tried to keep the setup fairly clean,
and have made a super inelegant CLI to help this effort.

```sh
$ cargo run <year> <day>
part one: test

part two: test

```

### Nim

```sh
$ nimble run day_file
# output logged below
```

## Languages Used

|year|language|
|---|---|
|2020|python|
|2021|rust|
|2022|nim|
|2023|node|
