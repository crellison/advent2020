import lists, strutils, sequtils, strformat

type NodeEntry = ref object
  moved: bool
  value: int

proc partOne*(input: string): string =
  var linkedRing = initDoublyLinkedRing[NodeEntry]()
  let lines = input.split("\n")
  for line in lines:
    let nextNode = newDoublyLinkedNode[NodeEntry](NodeEntry(
      moved: false, value: parseInt(line)
    ))
    linkedRing.add(nextNode)

  var movedNodes = 0
  var currentNode = linkedRing.head
  for i in 0..high(lines):
    
    while currentNode.value.moved == true:
      currentNode = currentNode.next

    let delta = if currentNode.value.value > 0: currentNode.value.value else: abs(currentNode.value.value) + 1
    var newPrev = currentNode
    for _ in 1..delta:
      if currentNode.value.value > 0:
        newPrev = newPrev.next
      else:
        newPrev = newPrev.prev
    echo &"moving {currentNode.value.value} to position of {newPrev.value.value}"
    var lastPrev = currentNode.prev
    var lastNext = currentNode.prev
    var newNext = newPrev.next
    
    lastPrev.next = lastNext
    lastNext.prev = lastPrev

    newPrev.next = currentNode
    currentNode.prev = newPrev
    newNext.prev = currentNode
    currentNode.next = newNext
    currentNode.value.moved = true
    inc(movedNodes)

  result = "done one"

proc partTwo*(input: string): string =
  result = "done two"
