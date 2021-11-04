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

## Languages Used

|year|language|
|---|---|
|2020|python|
|2021|rust|
