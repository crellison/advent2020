import tables, strutils, sequtils

type
  Visible = enum
    unknown, visible, hidden
  Tree = object
    height: int
    visible: Visible
  TreeLoc = object
    x,y: int
  Forest = ref object
    maxX, maxY: int
    trees: ref Table[TreeLoc, Tree]

const DIRECTIONS = [
  TreeLoc(x: 1, y: 0),
  TreeLoc(x: -1, y: 0),
  TreeLoc(x: 0, y: -1),
  TreeLoc(x: 0, y: 1)]

proc newForest(input: seq[string]): Forest =
  result = Forest(
    maxX: input.high,
    maxY: input[0].high,
    trees: newTable[TreeLoc, Tree]()
  )
  for x in 0..input.high:
    for y in 0..input[x].high:
      let isEdge = x == 0 or y == 0 or x == input.high or y == input[x].high
      result.trees[TreeLoc(x: x, y: y)] = Tree(
        height: parseInt($input[x][y]),
        visible: if isEdge: Visible.visible else: Visible.unknown
      )

method onEdge(forest: Forest, treeLoc: TreeLoc): bool {.base.} =
  doAssert forest.trees.hasKey(treeLoc)
  return treeLoc.x == 0 or treeLoc.y == 0 or treeLoc.x == forest.maxX or treeLoc.y == forest.maxY

method findBlockingTrees(forest: Forest, treeLoc: TreeLoc): array[4, TreeLoc] {.base.} =
  doAssert forest.trees.hasKey(treeLoc)
  doAssert not forest.onEdge(treeLoc)
  for i in 0..3:
    var nextLoc = TreeLoc(
      x: treeLoc.x + DIRECTIONS[i].x,
      y: treeLoc.y + DIRECTIONS[i].y
    )
    block findBlocking:
      while forest.trees.hasKey(nextLoc) and not forest.onEdge(nextLoc):
        if forest.trees[nextLoc].height >= forest.trees[treeLoc].height:
          break findBlocking
        nextLoc.x += DIRECTIONS[i].x
        nextLoc.y += DIRECTIONS[i].y
    result[i] = nextLoc

method isTreeVisible(forest: Forest, treeLoc: TreeLoc): bool {.base.} =
  doAssert forest.trees.hasKey(treeLoc)
  if forest.trees[treeLoc].visible == Visible.unknown:
    let blockingTrees = forest.findBlockingTrees(treeLoc)
    let treeHidden = blockingTrees.all(proc(blockLoc: TreeLoc): bool =
      forest.trees[blockLoc].height >= forest.trees[treeLoc].height
    )
    forest.trees[treeLoc].visible = if treeHidden: Visible.hidden else: Visible.visible
  result = forest.trees[treeLoc].visible == Visible.visible

method treeScenicScore(forest: Forest, treeLoc: TreeLoc): int {.base.} =
  doAssert forest.trees.hasKey(treeLoc)
  if forest.onEdge(treeLoc):
    return 0
  let blockingTrees = forest.findBlockingTrees(treeLoc)
  let distances = blockingTrees.map(proc(blockLoc: TreeLoc): int =
    max(abs(blockLoc.x - treeLoc.x), abs(blockLoc.y - treeLoc.y))
  )
  result = foldl(distances, a * b)

proc partOne*(input: string): int =
  let forest = newForest(input.split("\n"))
  for treeLoc in forest.trees.keys:
    if forest.isTreeVisible(treeLoc):
      result += 1

proc partTwo*(input: string): int =
  result = 0
  let forest = newForest(input.split("\n"))
  for treeLoc in forest.trees.keys:
    result = max(result, forest.treeScenicScore(treeLoc))
