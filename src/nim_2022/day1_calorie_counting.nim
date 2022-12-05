import strutils, math, sequtils

proc partOne*(input: string): int =
  result = 0
  for elf in input.split("\n\n"):
    let filteredFood = filter(elf.split("\n"), proc(x: string): bool = x != "")
    let elfFood = sum(filteredFood.map(parseInt))
    if elfFood > result:
      result = elfFood

proc partTwo*(input: string): int =
  var largest: array[3, int] = [0, 0, 0]
  for elf in input.split("\n\n"):
    let filteredFood = filter(elf.split("\n"), proc(x: string): bool = x != "")
    let elfFood = sum(filteredFood.map(parseInt))
    let minIndex = find(largest, min(largest))
    largest[minIndex] = max(largest[minIndex], elfFood)
  result = sum(largest)
