import strutils, sets, sequtils
import common

# a = 97
# A = 65

proc getPriority(letter: char): int =
  let letterCode = int(letter)
  if letterCode > 96:
    result = letterCode - 96
  else:
    result = 26 + letterCode - 64

proc getItemOnBothSides(rucksackItems: string): char =
  let halfItemCount = int(rucksackItems.len() / 2)
  let
    first = toHashSet(rucksackItems.substr(0, halfItemCount - 1))
    second = toHashSet(rucksackItems.substr(halfItemCount))

  var sharedItems = first * second
  if sharedItems.len() > 1:
    raise newException(RangeDefect, "unexpected count of shared characters")
  result = sharedItems.pop()

proc buildRucksackGroups(input: string): string =
  let rucksackItems = input.split("\n").map(proc(l: string): HashSet[char] = toHashSet(l))
  for items in common.chunked(rucksackItems, 3):
    var sharedLetters = foldl(items, a * b)
    if sharedLetters.len() != 1:
      raise newException(RangeDefect, "expected each group to share exactly one item")
    result.add(sharedLetters.pop())

proc partOne*(input: string): int =
  result = foldl(input.split("\n"), a + getPriority(getItemOnBothSides(b)), 0)

proc partTwo*(input: string): int =
  result = foldl(buildRucksackGroups(input), a + getPriority(b), 0)
