import sets

proc findUniquePacket(input: string, packetLen: Positive): int =
  var packetWindow = input[0..packetLen-1]
  for i in packetLen..input.high:
    if len(toHashSet(packetWindow)) == packetLen:
      result = i
      break
    packetWindow[i mod packetLen] = input[i]


proc partOne*(input: string): int =
  result = findUniquePacket(input, 4)

proc partTwo*(input: string): int =
  result = findUniquePacket(input, 14)
