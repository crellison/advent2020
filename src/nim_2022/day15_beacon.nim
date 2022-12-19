import tables, strutils, re, sequtils, logging

var logger = newConsoleLogger()

type
  PositionType = enum
    BEACON, EMPTY, SENSOR, UNKNOWN
  BeaconMap = ref object
    sensors: Table[(int, int), int]
    beacons: seq[(int, int)]
    xMin, xMax, yMin, yMax: int

proc manhattanDistance(x,y: (int, int)): int =
  result = abs(x[0] - y[0]) + abs(x[1] - y[1])

proc `$`(positionType: PositionType): string =
  result = case positionType:
    of PositionType.UNKNOWN: "."
    of PositionType.EMPTY: "#"
    of PositionType.BEACON: "B"
    of PositionType.SENSOR: "S"

proc `$`(beaconMap: BeaconMap): string =
  for x in beaconMap.xMin..beaconMap.xMax:
    var line = ""
    for y in beaconMap.yMin..beaconMap.yMax:
      if beaconMap.sensors.contains((x,y)):
        line.add("S")
      elif beaconMap.beacons.contains((x,y)):
        line.add("B")
      else:
        var nextToken = "."
        for sensor, dist in beaconMap.sensors.pairs:
          if manhattanDistance(sensor, (x,y)) <= dist:
            nextToken = "#"
        line.add(nextToken)
    echo line

method addToMap(beaconMap: BeaconMap, sensorLoc, beaconLoc: (int, int)): void {.base.}=
  let dist = manhattanDistance(sensorLoc, beaconLoc)
  beaconMap.xMin = min(beaconMap.xMin, sensorLoc[0] - dist)
  beaconMap.xMax = max(beaconMap.xMax, sensorLoc[0] + dist)
  beaconMap.yMin = min(beaconMap.yMin, sensorLoc[1] - dist)
  beaconMap.yMax = max(beaconMap.yMax, sensorLoc[1] + dist)

  beaconMap.sensors[sensorLoc] = dist
  beaconMap.beacons.add(beaconLoc)

method isOccupiedSpace(beaconMap: BeaconMap, position: (int, int)): bool {.base.} =
  result = false
  for sensor, dist in beaconMap.sensors.pairs:
    if manhattanDistance(sensor, position) <= dist:
      result = true
      break

method canCellContainBeacon(beaconMap: BeaconMap, position: (int, int)): bool {.base.} =
  result = true
  if not beaconMap.beacons.contains(position):
    if beaconMap.isOccupiedSpace(position):
      result = false

proc partOne*(input: string): int =
  var beaconMap = BeaconMap(xMin: high(int), xMax: low(int), yMin: high(int), yMax: low(int))
  for line in input.split("\n"):
    let numbers = line.findAll(re"(\-?\d+)").map(parseInt)
    beaconMap.addToMap((numbers[0], numbers[1]), (numbers[2], numbers[3]))
  
  logging.log(lvlDebug, beaconMap)
  let rowToCheck = 2000000 # 10

  for x in beaconMap.xMin..beaconMap.xMax:
    if not beaconMap.canCellContainBeacon((x, rowToCheck)):
        inc(result)

proc partTwo*(input: string): int =
  var beaconMap = BeaconMap(xMin: high(int), xMax: low(int), yMin: high(int), yMax: low(int))
  for line in input.split("\n"):
    let numbers = line.findAll(re"(\-?\d+)").map(parseInt)
    beaconMap.addToMap((numbers[0], numbers[1]), (numbers[2], numbers[3]))

  let minRange = 0
  let maxRange = 4000000 # 20

  block findDistressBeacon:
    for sensor, dist in beaconMap.sensors.pairs:
      let outerDist = dist + 1
      for xDelta in 0..outerDist:
        let yDelta = outerDist - xDelta
        let pointsToCheck = [
          (sensor[0] + xDelta, sensor[1] + yDelta),
          (sensor[0] + xDelta, sensor[1] - yDelta),
          (sensor[0] - xDelta, sensor[1] + yDelta),
          (sensor[0] - xDelta, sensor[1] - yDelta),
        ]
        for point in pointsToCheck:
          if point[0] in minRange..maxRange and point[1] in minRange..maxRange:
            if not beaconMap.beacons.contains(point) and beaconMap.canCellContainBeacon(point):
              result = point[0] * maxRange + point[1]
              break findDistressBeacon
