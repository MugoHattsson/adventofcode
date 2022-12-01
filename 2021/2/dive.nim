import std/strutils

var line: string
var horizontal = 0
var depth = 0
var command = ""
var units = 0

while readLine(stdin, line):
    var temp = split(line, " ")
    command = temp[0]
    units = parseInt(temp[1])

    case command:
        of "forward":
            horizontal += units
        of "down":
            depth += units
        of "up":
            depth -= units
        else:
            discard

    
echo "Horizontal: ", horizontal
echo "Depth: ", depth
echo "Result: ", horizontal * depth
