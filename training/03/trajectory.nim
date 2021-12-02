import std/sequtils

var line: string
var lines: seq[string]

while readLine(stdin, line):
    lines.add(line)

var width = len(lines[0])
var length = len(lines)

proc count_slope_crashes(right: int, down: int, map: seq[string]): int =
    var row = 0
    var col = 0
    var crashes = 0

    while row < length:
        if lines[row][col] == '#':
            inc(crashes)
        row += down
        col = (col + right) mod width
    
    return crashes


var crashes: seq[int]

crashes.add(count_slope_crashes(1, 1, lines))
crashes.add(count_slope_crashes(3, 1, lines))
crashes.add(count_slope_crashes(5, 1, lines))
crashes.add(count_slope_crashes(7, 1, lines))
crashes.add(count_slope_crashes(1, 2, lines))

echo foldl(crashes, a * b)