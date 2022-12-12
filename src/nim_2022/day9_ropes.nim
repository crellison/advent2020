import common
import strutils, sets, strformat

type Rope = ref object
  body: seq[common.Point2D]

method tail(rope: Rope): common.Point2D {.base.} =
  result = rope.body[rope.body.high]

method head(rope: Rope): common.Point2D {.base.} =
  result = rope.body[0]

method movePart(rope: Rope, move: common.Point2D, index: Natural): void {.base.} =
  rope.body[index] = move + rope.body[index]

# moves head and returns new tail location
method moveHead(rope: Rope, direction: common.Direction2D): common.Point2D {.base.} =
  let delta = common.parseDelta(direction)
  rope.movePart(delta, 0)
  for i in 1..rope.body.high:
    if common.stepsAway(rope.body[i], rope.body[i-1]) > 1:
      let dx = rope.body[i-1].x - rope.body[i].x
      let dy = rope.body[i-1].y - rope.body[i].y
      let move = common.Point2D(
        x: if dx != 0: int(dx / abs(dx)) else: 0,
        y: if dy != 0: int(dy / abs(dy)) else: 0
      )
      rope.movePart(move, i)

  result = rope.tail()

proc getTailPath(input: string, rope: Rope): seq[string] =
  result.add($rope.tail())
  for line in input.split("\n"):
    let direction: common.Direction2D = case line[0]:
      of 'R':
        common.Direction2D.E
      of 'L':
        common.Direction2D.W
      of 'U':
        common.Direction2D.N
      of 'D':
        common.Direction2D.S
      else:
        raise newException(ValueError, &"Unexpected direction for line: {line}")
    let steps = parseInt($line[2..line.high])
    for i in 1..steps:
      result.add($rope.moveHead(direction))
  


proc partOne*(input: string): int =
  var rope = Rope(
    body: @[common.Point2D(x: 0, y: 0), common.Point2D(x: 0, y: 0)]
  )
  result = toHashSet(getTailPath(input, rope)).len()

proc partTwo*(input: string): int =
  var rope = Rope(
    body: @[
      common.Point2D(x: 0, y: 0),
      common.Point2D(x: 0, y: 0),
      common.Point2D(x: 0, y: 0),
      common.Point2D(x: 0, y: 0),
      common.Point2D(x: 0, y: 0),
      common.Point2D(x: 0, y: 0),
      common.Point2D(x: 0, y: 0),
      common.Point2D(x: 0, y: 0),
      common.Point2D(x: 0, y: 0),
      common.Point2D(x: 0, y: 0),
    ]
  )
  result = toHashSet(getTailPath(input, rope)).len()
