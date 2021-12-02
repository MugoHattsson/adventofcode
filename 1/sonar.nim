import std/strutils


proc get_depth(): int =
    try:
        result = parseInt(readLine(stdin))
    except:
        result = -1

var prev: int = -1
var current = get_depth()
var counter = 0

while current != -1:
    if current > prev and prev != -1:
        echo prev, " ", current, " bigger"
        inc(counter)
    else:
        echo prev, " ", current, " smaller"

    prev = current
    current = get_depth()
    
echo counter