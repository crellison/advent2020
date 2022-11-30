# Package

version = "0.0.1"
author = "Cole Ellison"
description = "AoC Solutions"
license = "MIT"
srcDir = "src/nim_2022"
binDir = "nim_bin_2022"
bin = @[]

for file in listFiles(srcDir):
  if "day" in file:
    bin.add file[13..^5]


# Deps

requires "nim >= 1.6.10"

# Scripts

task clean, "wipes the bin":
  exec "rm -rf " & binDir
