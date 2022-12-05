import strutils

# A - 65
# X - 88

proc partOne*(input: string): int =
  for line in input.split("\n"):
    let opponentThrow = int(char(line[0]))
    let yourThrow = int(char(line[2]))
    result += yourThrow - 87
    result += 3 * ((yourThrow - opponentThrow - 1) mod 3)

proc partTwo*(input: string): int =
  for line in input.split("\n"):
    let opponentThrow = int(char(line[0]))
    let intendedOutcome = int(char(line[2]))
    result += (intendedOutcome - 88) * 3
    let yourThrow = 1 + (int(opponentThrow) + int(intendedOutcome) - 1) mod 3
    result += yourThrow
