import system/io, strutils, math, sequtils

proc partOne*(input: string) =
  var largest = 0
  for elf in input.split("\n\n"):
    let filteredFood = filter(elf.split("\n"), proc(x: string): bool = x != "")
    let elfFood = sum(filteredFood.map(parseInt))
    if elfFood > largest:
      largest = elfFood
  echo largest

proc partTwo*(input: string) =
  var largest: array[3, int] = [0,0,0]
  for elf in input.split("\n\n"):
    let filteredFood = filter(elf.split("\n"), proc(x: string): bool = x != "")
    let elfFood = sum(filteredFood.map(parseInt))
    let minIndex = find(largest, min(largest))
    largest[minIndex] = max(largest[minIndex], elfFood)
  echo sum(largest)

proc main() =
  var exitCode = QuitSuccess

  # relative to package root
  const input = readFile("./input/2022/1-1.txt")
  for part in [partOne, partTwo]:
    try:
      part(input)
    except Exception:
      exitCode = QuitFailure
  quit(exitCode)


when isMainModule:
  try:
    main()
  except Exception as error:
    echo error.msg
