import system/io, strutils, math

proc partOne*(input: string) =
  var largest: int = 0
  var current: int = 0
  for line in input.split("\n"):
    if line == "":
      largest = max(largest, current)
      current = 0
    else:
      current += parseInt(line)
  echo largest

proc partTwo*(input: string) =
  var largest: array[3, int] = [0,0,0]
  var current: int = 0
  for line in input.split("\n"):
    if line == "":
      let newMin = max(min(largest), current)
      if find(largest, newMin) == -1:
        let changeIndex = find(largest, min(largest))
        largest[changeIndex] = newMin
      current = 0
    else:
      current += parseInt(line)
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
