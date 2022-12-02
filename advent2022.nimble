# Package

version = "0.0.1"
author = "Cole Ellison"
description = "AoC Solutions"
license = "MIT"
srcDir = "src"
binDir = "nim_bin"
bin = @["advent"]

# Deps

requires "nim >= 1.6.10"
requires "docopt >= 0.7.0"

# Scripts

task clean, "wipes the bin":
  exec "rm -rf " & binDir
