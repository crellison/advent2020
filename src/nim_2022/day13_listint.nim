import strutils, sequtils, strformat, algorithm

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
  # echo &"INT COMPARE: {left} < {right}"
  result = case right - left:
    of 1..high(int):
      Ordered.yes
    of low(int).. -1:
      Ordered.no
    else:
      Ordered.unknown 

# only pass in lines of the form [<tokens>]
proc seqOrIntCmp(left, right: SeqOrInt): Ordered =
  # echo &"comparing {left} {left.t} to {right} {right.t}"
  
  if left.t == PacketType.INT and right.t == PacketType.INT:
    result = intOrdered(left.i, right.i)
  # convert non seq to seq
  elif right.t == PacketType.INT:
    if left.s.len() == 0:
      result = Ordered.yes
    else:
      result = seqOrIntCmp(left, buildSeq(&"[{right}]"))
      # result = if intOrdered(left.s[0].i, right.i) == Ordered.yes: Ordered.yes else: Ordered.no
  elif left.t == PacketType.INT:
    if right.s.len() == 0:
      result = Ordered.no
    else:
      result = seqOrIntCmp(buildSeq(&"[{left}]"), right)
      # result = if intOrdered(left.i, right.s[0].i) == Ordered.no: Ordered.no else: Ordered.yes
  else:
    result = Ordered.unknown
    var cursor = 0
    while cursor <= left.s.high and result == Ordered.unknown:
      if right.s.high < cursor:
        result = Ordered.no
        break
      result = seqOrIntCmp(left.s[cursor], right.s[cursor])
      inc(cursor)
    if result == Ordered.unknown:
      result = Ordered.yes

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
