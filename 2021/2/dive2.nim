import std/strutils

var line: string
var horizontal = 0
var depth = 0
var aim = 0
var command = ""
var units = 0

while readLine(stdin, line):
    var temp = split(line, " ")
    command = temp[0]
    units = parseInt(temp[1])

    case command:
        of "forward":
            horizontal += units
            depth = depth + units * aim
        of "down":
            aim += units
        of "up":
            aim -= units
        else:
            discard

    
echo "Horizontal: ", horizontal
echo "Depth: ", depth
echo "Result: ", horizontal * depth