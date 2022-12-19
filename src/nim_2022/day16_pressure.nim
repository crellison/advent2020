import tables, re, strutils, strformat, sequtils, logging, algorithm

var logger = newConsoleLogger()

type
  Valve = ref object
    open: bool
    rate: int
    id: string
    edges: seq[string]
  ValveMap = ref object
    map: Table[string, Valve]
    distances: Table[(string, string), int]

method contains(valveMap: ValveMap, item: string): bool {.base.} =
  result = valveMap.map.contains(item)

method minSteps(valveMap: ValveMap, a, b: string, seen: seq[string] = @[]): int {.base.} =
  doAssert(valveMap.contains(a) and valveMap.contains(b), &"{a} or {b} not in map")
  if valveMap.distances.contains((a,b)):
    return valveMap.distances[(a,b)]
  
  if a == b:
    result = 0
  elif valveMap.map[a].edges.contains(b):
    result = 1
  else:
    let lengths = valveMap.map[a].edges.filter(
        proc(c: string): bool = not (c in seen)
      ).map(
        proc(c: string): int = valveMap.minSteps(c, b, concat(seen, @[a]))
      )
    if len(lengths) == 0:
      # found a bad path
      result = len(valveMap.map) + 1
    else:
      result = 1 + min(lengths)
  if result <= len(valveMap.map):
    valveMap.distances[(a,b)] = result
    valveMap.distances[(b,a)] = result

method flowRate(valveMap: ValveMap): int {.base.}=
  for key in valveMap.map.keys:
    if valveMap.map[key].open and valveMap.map[key].rate > 0:
      result.inc(valveMap.map[key].rate)

method closedValves(valveMap: ValveMap): seq[string] {.base.}=
  for key in valveMap.map.keys:
    if not valveMap.map[key].open and valveMap.map[key].rate > 0:
      result.add(key)

proc `$`(valve: Valve): string =
  return &"Valve {valve.id} has flow rate={valve.rate}; connected to {valve.edges}; open={valve.open}"

proc buildValveMap(input: string): ValveMap =
  result = ValveMap()
  for line in input.split("\n"):
    let valveId = line[6..7]
    let rate = parseInt(line.findAll(re"(\d+)")[0])
    let edgeIndexStart = line.findBounds(re"tunnels? leads? to valves? ")[1]
    let edges = line[edgeIndexStart+1..high(line)].split(", ")
    logger.log(lvlDebug, &"{valveId} connected to {$edges}")
    result.map[valveId] = Valve(open: false, rate: rate, id: valveId, edges: edges)
  var checkedNodes: seq[string] = @[]
  for a in result.map.keys:
    for b in result.map.keys:
      if not (b in checkedNodes):
        discard result.minSteps(a,b)
        checkedNodes.add(b)
  

proc pressureRelease(valveMap: ValveMap, current: string, stepsLeft: int = 30): (int, seq[string]) =
  result = (0, @[])
  doAssert valveMap.contains(current)
  let valvesToOpen = valveMap.closedValves().filter(
    proc(x: string): bool = valveMap.minSteps(current, x) + 2 < stepsLeft
  )
  # logger.log(lvlDebug, &"------- {stepsLeft} steps left. at {current} -------")
  # logger.log(lvlDebug, valvesToOpen.join(","))
  if len(valvesToOpen) == 0 or stepsLeft <= 0:
    return
    # logger.log(lvlDebug, &"done traversing graph")
  else:
    for valve in valvesToOpen:
      let mapCopy = deepCopy(valveMap)
      mapCopy.map[valve].open = true
      let stepDelta = mapCopy.minSteps(current, valve) + 1
      let stepRelease = mapCopy.map[valve].rate * (stepsLeft - stepDelta)
      # logger.log(lvlDebug, &"opening {valve} with {stepsLeft - stepDelta} steps left adds {stepRelease} to total")
      let (nextRelease, stepsSeen) = pressureRelease(mapCopy, valve, stepsLeft - stepDelta)
      if result[0] < stepRelease + nextRelease:
        result = (stepRelease + nextRelease, concat(stepsSeen, @[valve]))

  if result[1].len() >= 5:
    let reversedPath = result[1].reversed().join(",")
    logger.log(lvlDebug, &"path {reversedPath} has weight {result[0]}; steps left: {stepsLeft}")

proc pressureRelease2(valveMap: ValveMap, current: (string, string), stepsLeft: (int, int) = (26, 26)): (int, seq[string]) =
  doAssert valveMap.contains(current[0])
  doAssert valveMap.contains(current[1])
  let valvesToOpen = valveMap.closedValves().filter(
    proc(valve: string): bool = valveMap.minSteps(current[0], valve) + 2 < stepsLeft[0]
  )
  let otherValvesToOpen = valveMap.closedValves().filter(
    proc(valve: string): bool = valveMap.minSteps(current[1], valve) + 2 < stepsLeft[1]
  )
  logger.log(lvlDebug, &"------- {stepsLeft} steps left. at {current} -------")
  logger.log(lvlDebug, valvesToOpen.join(","))
  if (len(valvesToOpen) == 0 and len(otherValvesToOpen) == 0) or (stepsLeft[0] <= 0 and stepsLeft[1] <= 0):
    result = (0, @[])
  else:
    logger.log(lvlDebug, valvesToOpen.join(","))
    logger.log(lvlDebug, otherValvesToOpen.join(","))

    if len(valvesToOpen) == 0:
      return pressureRelease(valveMap, current[1], stepsLeft[1])
    elif len(otherValvesToOpen) == 0:
      return pressureRelease(valveMap, current[0], stepsLeft[0])
    
    for valve in valvesToOpen:
      for otherValve in otherValvesToOpen:
        if valve == otherValve:
          continue
        let mapCopy = deepCopy(valveMap)
        mapCopy.map[valve].open = true
        mapCopy.map[otherValve].open = true
        let stepDelta = mapCopy.minSteps(current[0], valve) + 1
        let otherStepDelta = mapCopy.minSteps(current[1], otherValve) + 1
        let stepRelease = mapCopy.map[valve].rate * (stepsLeft[0] - stepDelta)
        let otherStepRelease = mapCopy.map[otherValve].rate * (stepsLeft[1] - otherStepDelta)
        let (nextRelease, stepsSeen) = pressureRelease2(mapCopy, (valve, otherValve), (stepsLeft[0] - stepDelta, stepsLeft[1] - otherStepDelta))
        if result[0] < stepRelease + otherStepRelease + nextRelease:
          logger.log(lvlDebug, &"moving {stepDelta} steps from {current} to {valve} adds {stepRelease} to total")
          result = (stepRelease + nextRelease + otherStepRelease, concat(stepsSeen, @[valve, otherValve]))


proc partOne*(input: string): int =
  let valveMap = buildValveMap(input)
  let (released, route) = pressureRelease(valveMap, "AA")
  logger.log(lvlDebug, route.reversed().join(","))
  result = released

proc partTwo*(input: string): int =
  let valveMap = buildValveMap(input)

  let (released, route) = pressureRelease2(valveMap, ("AA", "AA"))
  logger.log(lvlDebug, route.reversed().join(","))
  result = released

# 2334 low
