import strutils, sequtils, strformat, algorithm, logging

var logger = newConsoleLogger()

type Ordered = enum
  yes, no, unknown

type PacketType = enum
  INT, SEQ

type SeqOrInt = ref object
  t: PacketType
  s: seq[SeqOrInt]
  i: int

proc `$`(s: SeqOrInt): string =
  if s.t == PacketType.SEQ:
    result.add("[")
    result.add(s.s.map(proc(e: SeqOrInt): string = $e).join(","))
    result.add("]")
  else:
    result = &"{s.i}"

proc buildSeq(tokens: string): SeqOrInt =
  result = SeqOrInt()
  if not tokens.startsWith('['):
    result.t = PacketType.INT
    # assume just int
    result.i = parseInt(tokens)
  else:
    result.t = PacketType.SEQ
    var startCursor = 1
    var endCursor = 0
    var braceTally = 1

    while braceTally != 0 and endCursor < tokens.high:
      inc(endCursor)
      case tokens[endCursor]:
        of ']':
          braceTally -= 1
          if braceTally == 0 and endCursor - startCursor >= 1:
            result.s.add(buildSeq(tokens.substr(startCursor,endCursor-1)))
            startCursor = endCursor + 1
        of '[':
          braceTally += 1
        of ',':
          if braceTally == 1:
            result.s.add(buildSeq(tokens.substr(startCursor, endCursor-1)))
            startCursor = endCursor + 1
        else:
          # just part of the sequence
          continue

proc intOrdered(left: int, right: int): Ordered =
  logger.log(lvlDebug, &"INT COMPARE: {left} < {right}")
  result = case right - left:
    of 1..high(int):
      logger.log(lvlDebug, "left side is smaller, so we are good")
      Ordered.yes
    of low(int).. -1:
      logger.log(lvlDebug, "right side is smaller, so we are bad")
      Ordered.no
    else:
      logger.log(lvlDebug, "who knows, let's keep going")
      Ordered.unknown 

# only pass in lines of the form [<tokens>]
proc seqOrIntCmp(left, right: SeqOrInt): Ordered =
  result = Ordered.unknown
  logger.log(lvlDebug, &"compare {left} to {right}")
  if left.t == PacketType.INT and right.t == PacketType.INT:
    result = intOrdered(left.i, right.i)
  elif right.t == PacketType.INT:
    result = seqOrIntCmp(left, buildSeq(&"[{right}]"))
  elif left.t == PacketType.INT:
    result = seqOrIntCmp(buildSeq(&"[{left}]"), right)
  else:
    block findOrdering:
      for i in 0..min(left.s.high(), right.s.high()):
        result = seqOrIntCmp(left.s[i], right.s[i])
        if result != Ordered.unknown:
          break findOrdering
    if result == Ordered.unknown and left.s.high() != right.s.high():
      result = if left.s.high() > right.s.high():
        logger.log(lvlDebug, "left is longer, so it's a no")
        Ordered.no
      else:
        logger.log(lvlDebug, "right is longer, so we are good!")
        Ordered.yes

proc `<`(a,b: SeqOrInt): bool =
  result = if seqOrIntCmp(a,b) == Ordered.yes: true else: false
  
proc partOne*(input: string): int =
  let listPairs = input.split("\n\n").map(proc(group: string): (string, string) = 
    (group.split("\n")[0], group.split("\n")[1])
  )
  for i in 0..listPairs.high:
    let left = buildSeq(listPairs[i][0])
    let right = buildSeq(listPairs[i][1])
    let orderedResult = seqOrIntCmp(left, right)
    if orderedResult != Ordered.no:
      result += i + 1

proc partTwo*(input: string): int =
  let dividerPackets = ["[[2]]","[[6]]"].map(buildSeq)
  var packetList = input.split("\n").filter(proc(line: string): bool = line != "").map(buildSeq)
  for packet in dividerPackets:
    packetList.add(packet)

  packetList.sort()
  result = foldl(dividerPackets, a * (packetList.find(b) + 1), 1)
