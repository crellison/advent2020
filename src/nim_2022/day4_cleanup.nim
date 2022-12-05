import re, strutils

proc getRanges(line: string): array[4, int] =
  let matchedItems = line.findAll(re"(\d+)")
  for i in 0..3:
    result[i] = parseInt(matchedItems[i])

proc doPairsContain(lowA, highA, lowB, highB: int): bool =
  return (lowA <= lowB and highA >= highB) or (lowA >= lowB and highA <= highB)

proc doPairsOverlap(lowA, highA, lowB, highB: int): bool =
  return (lowA <= lowB and highA >= lowB) or
    (lowA <= highB and highA >= highB) or
    doPairsContain(lowA, highA, lowB, highB)

proc partOne*(input: string): int =
  for line in input.split("\n"):
    if line == "":
      continue
    let ranges = getRanges(line)
    if doPairsContain(ranges[0], ranges[1], ranges[2], ranges[3]):
        result += 1

proc partTwo*(input: string): int =
  for line in input.split("\n"):
    if line == "":
      continue
    let ranges = getRanges(line)
    if doPairsoverlap(ranges[0], ranges[1], ranges[2], ranges[3]):
        result += 1
