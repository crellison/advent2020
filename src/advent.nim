let doc = """
Advent of Nim

Usage:
  advent <year> <day> <inputId> [--debug]

Types:
  year        int (only 2022 currently)
  day         int 1..25
  inputId     id for input file (<day>-<inputId>.txt)

Options:
  -h --help     Show this screen.
  --version     Show version.
"""

import system/io, strformat, docopt, strutils, logging
import nim_2022/[
  day1_calorie_counting, day2_rock_paper_scissors, day3_rucksack, day4_cleanup, day5_stacks,
  day6_tuning, day7_terminal, day8_treehouse, day9_ropes, day10_signal, day11_monkey, day13_listint,
  day15_beacon, day16_pressure, day17_rocks, day18_faces, day20_decrypt]

proc main() =
  let args = docopt(doc, version = "Advent of Nim 0.0.1")
  let
    year = $args["<year>"]
    day = $args["<day>"]
    inputId = $args["<inputId>"]
    debug = parseBool($args["--debug"])

  setLogFilter(if debug: lvlDebug else: lvlError)
  var input = readFile(&"./input/{year}/{day}-{inputId}.txt")
  input.removeSuffix("\n")
  case year:
    of "2022":
      case day:
        of "1":
          echo day1_calorie_counting.partOne(input)
          echo day1_calorie_counting.partTwo(input)
        of "2":
          echo day2_rock_paper_scissors.partOne(input)
          echo day2_rock_paper_scissors.partTwo(input)
        of "3":
          echo day3_rucksack.partOne(input)
          echo day3_rucksack.partTwo(input)
        of "4":
          echo day4_cleanup.partOne(input)
          echo day4_cleanup.partTwo(input)
        of "5":
          echo day5_stacks.partOne(input)
          echo day5_stacks.partTwo(input)
        of "6":
          echo day6_tuning.partOne(input)
          echo day6_tuning.partTwo(input)
        of "7":
          echo day7_terminal.partOne(input)
          echo day7_terminal.partTwo(input)
        of "8":
          echo day8_treehouse.partOne(input)
          echo day8_treehouse.partTwo(input)
        of "9":
          echo day9_ropes.partOne(input)
          echo day9_ropes.partTwo(input)
        of "10":
          echo day10_signal.partOne(input)
          echo day10_signal.partTwo(input)
        of "11":
          echo day11_monkey.partOne(input)
          echo day11_monkey.partTwo(input)
        of "13":
          echo day13_listint.partOne(input)
          echo day13_listint.partTwo(input)
        of "15":
          echo day15_beacon.partOne(input)
          echo day15_beacon.partTwo(input)
        of "16":
          echo day16_pressure.partOne(input)
          echo day16_pressure.partTwo(input)
        of "17":
          echo day17_rocks.partOne(input)
          echo day17_rocks.partTwo(input)
        of "18":
          echo day18_faces.partOne(input)
          echo day18_faces.partTwo(input)
        of "20":
          echo day20_decrypt.partOne(input)
          echo day20_decrypt.partTwo(input)
        else:
          echo &"solution not available for {year}-{day}"
    else:
      echo &"no solutions for year {year}"


when isMainModule:
  try:
    main()
  except Exception as error:
    echo error.msg
