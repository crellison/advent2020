import strutils, strformat

type Register = ref object
  cycle: int
  X: int

method runCycle(r: Register, deltaX: int): void {.base.} =
  let index = (r.cycle - 1) mod 40
  let delta = abs(r.X - index)
  if index == 0:
    stdout.write("\n")
  if delta <= 1:
    stdout.write("#")
  else:
    stdout.write(".")
  r.cycle += 1
  r.X += deltaX


proc partOne*(input: string): int =
  var register = Register(X: 1, cycle: 1)
  for line in input.split("\n"):
    if register.cycle >= 20 and (register.cycle - 20) mod 40 == 0:
      result += register.cycle * register.X
    case line[0..3]:
      of "noop":
        register.runCycle(0)
      of "addx":
        register.runCycle(0)
        if register.cycle >= 20 and (register.cycle - 20) mod 40 == 0:
          result += register.cycle * register.X
        register.runCycle(parseInt(line[5..line.high]))
      else:
        raise newException(ValueError, &"unknown command on line: {line}")

proc partTwo*(input: string): string =
  result = "see above"
