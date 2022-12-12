import strformat

type
  Point2D* = ref object
    x*, y*: int
  Direction2D* = enum
    N, NE, E, SE, S, SW, W, NW

proc `+`*(a, b: Point2D): Point2D =
  result = Point2D(x: a.x + b.x, y: a.y + b.y)

proc `$`*(pt: Point2D): string =
  result = &"({pt.x}, {pt.y})"

proc stepsAway*(a, b: Point2D): int =
  result = max(abs(a.x - b.x), abs(a.y - b.y))

proc parseDelta*(direction: Direction2D): Point2D =
  var dx, dy = 0
  if direction == Direction2D.N or direction == Direction2D.NE or direction == Direction2D.NW:
    dy = 1
  if direction == Direction2D.S or direction == Direction2D.SE or direction == Direction2D.SW:
    dy = -1
  if direction == Direction2D.E or direction == Direction2D.SE or direction == Direction2D.NE:
    dx = 1
  if direction == Direction2D.W or direction == Direction2D.SW or direction == Direction2D.NW:
    dx = -1
  result = Point2D(x: dx, y: dy)

# Returns an iterable that steps through an interable in chunkSize sequences
iterator chunked*[T](range: openArray[T], chunkSize: Positive = 1): seq[T] =
  var i = 0
  while i < range.len():
    var nextSeq: seq[T]
    for j in 0..chunkSize-1:
      if i+j < range.len():
        nextSeq.add(range[i+j])
    yield nextSeq
    inc(i, chunkSize)

export chunked, parseDelta
