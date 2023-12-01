import strutils, sequtils, re, strscans, strformat, algorithm

type Monkey = ref object
  items: seq[uint64]
  itemsInspected: int
  # op is mult, value, worryDelta
  opParams: (bool, uint64, uint64)
  # divisor, ifTrue, ifFalse
  nextParams: (uint64, int, int) 


method op(monkey: Monkey, item: uint64): uint64 {.base.} =
  let isMult = monkey.opParams[0]
  let opVal = if monkey.opParams[1] == 0: item else: monkey.opParams[1]
  let worryDelta = monkey.opParams[2]
  result = if isMult: item * opVal else: item + opVal
  if worryDelta != 1:
    result = result div worryDelta

method next(monkey: Monkey, item: uint64): int {.base.} =
  let (divisor, ifTrue, ifFalse) = monkey.nextParams
  result = if item mod divisor == 0: ifTrue else: ifFalse

proc buildMonkeyList(input: string, worryDelta: uint64): seq[Monkey] =
  for chunk in input.split("\n\n"):
    let lines = chunk.split("\n")
    let items = lines[1].findAll(re"\d+").map(proc(x: string): uint64 = uint64(parseInt(x)))
    let (_, opCode, opVal) = scanTuple(lines[2], "  Operation: new = old $c $i")
    let (_, divisor) = scanTuple(lines[3], "  Test: divisible by $i")
    let trueNextMonkey = parseInt(lines[4][^1..^1])
    let falseNextMonkey = parseInt(lines[5][^1..^1])
    
    result.add(Monkey(
      items: items,
      itemsInspected: 0,
      opParams: (opCode == '*', uint64(opVal), worryDelta),
      nextParams: (uint64(divisor), trueNextMonkey, falseNextMonkey)
    ))

proc monkeyBusiness(monkeyList: seq[Monkey], rounds: int): int =
  for j in 0..rounds-1:
    if j mod 1000 == 0 or j == 1 or j == 20:
      echo &"== After round {j} =="
      for index in 0..monkeyList.high():
        echo &"Monkey {index} inspected items {monkeyList[index].itemsInspected} times"
    for i in 0..high(monkeyList):
      monkeyList[i].itemsInspected += monkeyList[i].items.len()
      for item in monkeyList[i].items:
        let newLevel = monkeyList[i].op(item)
        let nextMonkey = monkeyList[i].next(newLevel)
        monkeyList[nextMonkey].items.add(newLevel)
      monkeyList[i].items = @[]

  var inspectedItems = monkeyList.map(proc(m: Monkey): int = m.itemsInspected)
  inspectedItems.sort()
  echo inspectedItems
  result = inspectedItems[^2] * inspectedItems[^1]
  
proc partOne*(input: string): int =
  var monkeyList = buildMonkeyList(input, 3)
  result = monkeyBusiness(monkeyList, 20)

proc partTwo*(input: string): int =
  var monkeyList = buildMonkeyList(input, 1)
  result = monkeyBusiness(monkeyList, 10000)
# 24299652916 (low)
# 32390460700 (high)
