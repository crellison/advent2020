import sequtils, strutils, strformat

type
  Rock = seq[(int, int)]
  ColumnCell = enum
    empty, rock
  ColumnRow = array[7, ColumnCell]
  Column = seq[ColumnRow]

proc `$`(cell: ColumnCell): string =
  result = case cell:
    of ColumnCell.empty: "."
    of ColumnCell.rock: "#"

proc `$`(column: Column): string =
  for i in high(column)..0:
    result.add(column[i].join(""))
    result.add("\n")

const rocks: array[5, Rock] = [
  zip(toSeq(0..3), newSeq[int](4)), # horizontal
  @[(1,0), (0,1), (1,1), (2,1), (1,2)], # plus
  @[(0,0), (1,0), (2,0), (2,1), (2,2)], # angle
  zip(newSeq[int](4), toSeq(0..3)), # vertical
  zip(@[1,1,0,0], @[1,0,1,0]), # square
]

proc rockLands(column: Column, rock: Rock, location: int, columnOffset: int = 0): bool =
  result = false
  if columnOffset == 0 and len(column) == 0:
    result = true
  elif columnOffset <= 0:
    for pebble in rock:
      let dropPoint = (location + pebble[0], len(column) + columnOffset + pebble[1] - 1)
      if dropPoint[1] <= high(column) and column[dropPoint[1]][dropPoint[0]] == ColumnCell.rock:
        return true

proc canRockShift(column: Column, rock: Rock, location: int, isLeft: bool, columnOffset: int = 0): bool =
  result = true
  let delta = if isLeft: -1 else: 1
  for pebble in rock:
    let shiftPoint = (location + pebble[0] + delta, len(column) + columnOffset + pebble[1])
    if not (shiftPoint[0] in 0..6):
      result = false
    elif shiftPoint[1] <= high(column) and column[shiftPoint[1]][shiftPoint[0]] == ColumnCell.rock:
      return false

proc rockFall(input: string, numRocks: int64): int =
  var rockIndex = 0
  var rockLocation = 2
  var rockOffset = 3
  var column: Column = @[
    [ColumnCell.rock, ColumnCell.rock, ColumnCell.rock, ColumnCell.rock, ColumnCell.rock, ColumnCell.rock, ColumnCell.rock]
  ]
  var dirOffset = 0
  var fallenRocks = 0
  while fallenRocks != numRocks:
    let dir = input[dirOffset]
    if canRockShift(column, rocks[rockIndex], rockLocation, dir == '<', rockOffset):
      inc(rockLocation, if dir == '<': -1 else: 1)
    if rockLands(column, rocks[rockIndex], rockLocation, rockOffset):
      let rockLocations = rocks[rockIndex].map(
        proc(coord: (int, int)): (int, int) = (coord[0] + rockLocation, len(column) + rockOffset + coord[1])
      )
      for pebble in rockLocations:
        while pebble[1] > high(column):
          column.add([ColumnCell.empty, ColumnCell.empty, ColumnCell.empty, ColumnCell.empty, ColumnCell.empty, ColumnCell.empty, ColumnCell.empty])
        column[pebble[1]][pebble[0]] = ColumnCell.rock
      # land the rock
      rockIndex = (rockIndex + 1) mod rocks.len()
      rockOffset = 3
      rockLocation = 2
      inc(fallenRocks)
    else:
      dec(rockOffset)

    dirOffset = (dirOffset + 1) mod len(input)
  result = len(column) - 1

proc partOne*(input: string): int =
  result = rockFall(input, 2022)

proc partTwo*(input: string): int =
  result = rockFall(input, 1000000000000)
