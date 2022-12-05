import strutils, sequtils, re

type
  Order = object
    numItems: Natural
    startStack, endStack: int

proc parseStacks(input: string): seq[seq[char]] =
  let stackInput = input.split("\n\n")[0].split("\n")
  let stackCount = parseInt(stackInput[stackInput.len() - 1][^2..^2])
  newSeq(result, stackCount)
  for i in 2..len(stackInput):
    let currentLine = stackInput[len(stackInput) - i]
    for j in 0..stackCount - 1:
      if currentLine[j * 4 + 1] != ' ':
        result[j].add(char(currentLine[j * 4 + 1]))

proc parseOrder(order: string): Order =
  let orderDirections = order.findAll(re"(\d+)")
  result = Order(
    numItems: parseInt(orderDirections[0]),
    startStack: parseInt(orderDirections[1]) - 1,
    endStack: parseInt(orderDirections[2]) - 1
  )

proc partOne*(input: string): string =
  var stacks = parseStacks(input)
  for order in input.split("\n\n")[1].split("\n").map(parseOrder):
    for _ in 1..order.numItems:
      stacks[order.endStack].add(stacks[order.startStack].pop())
  for column in stacks:
    result.add(column[column.len()-1])

proc partTwo*(input: string): string =
  var stacks = parseStacks(input)
  for order in input.split("\n\n")[1].split("\n").map(parseOrder):

    stacks[order.endStack] = concat(
      stacks[order.endStack],
      stacks[order.startStack][^order.numItems..^1])

    stacks[order.startStack] = stacks[order.startStack][0..^order.numItems+1]

  for column in stacks:
    result.add(column[column.len()-1])
