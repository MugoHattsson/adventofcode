from std/strutils import parseInt

proc find_2(lines: seq[int]): int =
    for i in 0 ..< high(lines):
        for j in 1 .. high(lines):
            if lines[i] + lines[j] == 2020:
                return lines[i] * lines[j] 

proc find_3(lines: seq[int]): int =
    for i in 0 ..< high(lines)-1:
        for j in 1 .. high(lines)-1:
            for k in 2 .. high(lines):
                if lines[i] + lines[j] + lines[k] == 2020:
                    return lines[i] * lines[j] * lines[k] 

var line: string
var lines: seq[int]

while readLine(stdin, line):
    lines.add(parseInt(line))

#echo lines
echo "Lines: ", len(lines)

echo find_2(lines)
echo find_3(lines)
