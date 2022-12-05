import strformat

# Returns an iterable that steps through an interable in chunkSize sequences
iterator chunked*[T](range: openArray[T], chunkSize: int = 1): seq[T] =
  if chunkSize < 1:
    raise newException(RangeDefect, &"expected range greater than 1. received: {chunkSize}")
  var i = 0
  while i < range.len():
    var nextSeq: seq[T]
    for j in 0..chunkSize-1:
      if i+j < range.len():
        nextSeq.add(range[i+j])
    yield nextSeq
    inc(i, chunkSize)

export chunked
