import strutils, strformat, sequtils, tables

type
  FileSystem = ref object
    dirName: string
    subDirs: seq[FileSystem]
    parent: FileSystem
    files: seq[File]

  File = object
    name: string
    size: int

const
  ROOT_PATH = ""
  TOTAL_SPACE = 70000000
  MIN_FREE_SPACE = 30000000

proc cd(fileSystem: FileSystem, targetDir: string): FileSystem =
  case targetDir:
    of "/":
      result = fileSystem
      while result.dirName != ROOT_PATH:
        result = result.parent
    of "..":
      result = fileSystem.parent
    else:
      let subDirNames = fileSystem.subDirs.map(proc(fs: FileSystem): string = fs.dirName)
      let childLocation = find(subDirNames, targetDir)
      if childLocation == -1:
        raise newException(RangeDefect, &"{fileSystem.dirName} does not contain subdir {targetDir}: {subDirNames}")
      else:
        result = fileSystem.subDirs[childLocation]

proc ls(fileSystem: FileSystem, lsOutput: seq[string]): FileSystem =
  for line in lsOutput:
    let lsParts = line.split(" ")
    case lsParts[0]:
    of "dir":
      let newDir = FileSystem(dirName: lsParts[1], parent: fileSystem)
      fileSystem.subDirs.add(newDir)
    else:
      let newFile = File(
        name: lsParts[1],
        size: parseInt(lsParts[0])
      )
      fileSystem.files.add(newFile)
  result = fileSystem

proc buildFileSystem(input: string): FileSystem =
  let fileSystem = FileSystem(dirName: ROOT_PATH)
  var currentLoc = fileSystem
  var commands = input.split("\n$ ")
  commands[0] = commands[0].substr(2)

  for commandBlock in commands:
    let commandCode = commandBlock[0..1]
    case commandCode:
      of "cd":
        let targetDir = commandBlock[3..commandBlock.high]
        currentLoc = cd(currentLoc, targetDir)
      of "ls":
        let lsItems = commandBlock.split("\n")
        currentLoc = ls(currentLoc, lsItems[1..lsItems.high])
      else:
        echo &"unrecognized command: {commandCode}"

  while currentLoc.dirName != ROOT_PATH:
    currentLoc = currentLoc.parent
  
  result = currentLoc

proc countDirSizes(fileSystem: FileSystem): TableRef[string, int] =
  let dirSizes: TableRef[string, int] = newTable[string, int]()

  proc buildSizesTable(fs: FileSystem, pathPrefix: string = ROOT_PATH) =
    let currentDirPath = &"{pathPrefix}{fs.dirName}"
    if not dirSizes.contains(currentDirPath):
      dirSizes[currentDirPath] = 0 

    for subDir in fs.subDirs:
      buildSizesTable(subDir, &"{currentDirPath}/")

    for file in fs.files:
      dirSizes[currentDirPath] += file.size

      for dir in dirSizes.keys:
        if currentDirPath.startsWith(dir) and dir != currentDirPath:
          dirSizes[dir] += file.size

  buildSizesTable(fileSystem)
  result = dirSizes

proc partOne*(input: string): int =
  let dirSizes = countDirSizes(buildFileSystem(input))
  for size in dirSizes.values:
    if size <= 100000:
      result += size

proc partTwo*(input: string): int =
  let dirSizes = countDirSizes(buildFileSystem(input))
  let currentFreeSpace = TOTAL_SPACE - dirSizes[ROOT_PATH]
  let spaceToRelease = MIN_FREE_SPACE - currentFreeSpace
  result = dirSizes[ROOT_PATH] 
  for dirSize in dirSizes.values:
    if dirSize > spaceToRelease and dirSize < result:
      result = dirSize
