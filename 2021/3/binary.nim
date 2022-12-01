import std/strutils
import std/sequtils
import std/sugar
import std/lists


var line: string
var lines: seq[string]

while readLine(stdin, line):
    lines.add(line)

var width = len(lines[0])
var length = len(lines)
var gamma: string
var epsilon: string

var num = 0
var counter = 0

for i in 0 ..< width:
    for line in lines:
        num = parseInt($line[i])
        
        if num == 1:
            inc(counter)
    
    if counter.toFloat > length/2:
        gamma.add('1')
        epsilon.add('0')
    else:
        gamma.add('0')
        epsilon.add('1')
    
    counter = 0

proc countEqual(lines: seq[string], index: int, equalChar: char): char =
    var counter = 0
    for line in lines:
        if line[index] == '1':
            inc(counter)
        else:
            dec(counter)

    if counter > 0:
        result = '1'
    elif counter < 0:
        result = '0'
    else:
        result = equalChar 

    return result



proc filterSeq(numbers: seq[string], mask: string, equalChar: char): int =
    var lines = numbers
    echo lines
    for i in 0..<len(mask):
        var j = 0
        var digit = mask[i]
        while j < len(lines) and len(lines) > 1:
            digit = countEqual(lines, i, equalChar)
            echo "j: ", j
            if lines[j][i] != digit:
                echo "Digit: ", digit, ", deleted: ", lines[j]
                lines.delete(j)
            else:
                inc(j)
        echo lines
    echo lines[0]
    return parseBinInt(lines[0])

# var o2 = filterSeq(lines, gamma, '1')
var co2 = filterSeq(lines, epsilon, '0')

echo co2

# echo o2, ", ", co2, ", ", o2*co2

echo gamma
echo epsilon

echo parseBinInt(gamma) * parseBinInt(epsilon)