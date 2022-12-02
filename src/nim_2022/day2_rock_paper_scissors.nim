import system/io, strutils

# A - 65
# X - 88

proc partOne*(input: string) =
  var score = 0
  for line in input.split("\n"):
    if line == "":
      continue
    let opponentThrow = int(char(line[0]))
    let yourThrow = int(char(line[2]))
    score += yourThrow - 87
    score += 3 * ((yourThrow - opponentThrow - 1) mod 3)
  echo score

proc partTwo*(input: string) =
  var score = 0
  for line in input.split("\n"):
    if line == "":
      continue
    let opponentThrow = int(char(line[0]))
    let intendedOutcome = int(char(line[2]))
    score += (intendedOutcome - 88) * 3
    let yourThrow = 1 + (int(opponentThrow) + int(intendedOutcome) - 1) mod 3
    score += yourThrow
  echo score

proc main() =
  var exitCode = QuitSuccess

  # relative to package root
  const input = readFile("./input/2022/2-1.txt")
  for part in [partOne, partTwo]:
    try:
      part(input)
    except Exception as exc:
      echo "bailing from exception"
      echo exc.msg
      exitCode = QuitFailure
  quit(exitCode)


when isMainModule:
  try:
    main()
  except Exception as error:
    echo error.msg
