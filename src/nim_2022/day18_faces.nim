import logging, strscans, strformat, sets, strutils, sequtils

var logger = newConsoleLogger()

type Point = (int, int, int)

proc parseLine(line: string): Point =
  let (success, x, y, z) = scanTuple(line, "$i,$i,$i")
  if success:
    result = (x,y,z)
  else:
    raise newException(ValueError, &"Unable to parse line: {line}")

proc getSharedFaces(point: Point): array[6, Point] =
  let (x,y,z) = point
  result = [
    (x, y, z + 1),  (x, y, z - 1),
    (x, y + 1, z),  (x, y - 1, z),
    (x + 1, y, z),  (x - 1, y, z),
  ]

proc partOne*(input: string): int =
  let points = toHashSet(input.split("\n").map(parseLine))
  for p in points:
    let sharedFaces = getSharedFaces(p)
    for point in sharedFaces:
      if not points.contains(point):
        inc(result)

proc partTwo*(input: string): int =
  var points = toHashSet(input.split("\n").map(parseLine))
  var maxXPoint = (low(int), 0, 0)
  for p in points:
    if maxXPoint[0] < p[0]:
      maxXPoint = p
  let knownExteriorPoint = (maxXPoint[0] + 1, maxXPoint[1], maxXPoint[2])
  logger.log(lvlDebug, &"known exterior point: {knownExteriorPoint}")
  # somehow scrub along the outer rim
  var knownEdges = toHashSet([knownExteriorPoint])
  var pointsToConsider = toHashSet(getSharedFaces(knownExteriorPoint))
  var checkedPoints: HashSet[Point]
  while len(pointsToConsider) != 0:
    let next = pointsToConsider.pop()
    checkedPoints = union(checkedPoints, toHashSet([next]))
    logger.log(lvlDebug, &"considering: {next}")

    # we don't need to do anything
    if next in points or next in knownEdges:
      continue

    var addToEdge = false
    var checkAdjacent = false
    for point in getSharedFaces(next):
      if point in points:
        addToEdge = true
        checkAdjacent = true
      if any(getSharedFaces(point), proc(x: Point): bool = x in points):
        checkAdjacent = true
    if addToEdge:
      logger.log(lvlDebug, &"it's an edge!")
      knownEdges = knownEdges + toHashSet([next])
    if checkAdjacent:
      logger.log(lvlDebug, &"adding neighbors to be checked")
      pointsToConsider = pointsToConsider + (toHashSet(getSharedFaces(next)) - checkedPoints)

  for p in points:
    let sharedFaces = getSharedFaces(p)
    for point in sharedFaces:
      if knownEdges.contains(point):
        inc(result)
