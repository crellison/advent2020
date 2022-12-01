import system/io

proc partOne*(input: string) =
  echo "done one"

proc partTwo*(input: string) =
  echo "done two"

proc main() =
  var exitCode = QuitSuccess

  # relative to package root
  const input = readFile("./input/${YEAR}/${DAY}-1.txt")
  for part in [partOne, partTwo]:
    try:
      part(input)
    except Exception as exc:
      echo "bailing from exception"
      echo exc.msg
      exitCode = QuitFailure
  quit(exitCode)


when isMainModule:
  try:
    main()
  except Exception as error:
    echo error.msg
