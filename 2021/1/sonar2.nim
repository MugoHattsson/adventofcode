import std/strutils


proc get_depth(): int =
    try:
        result = parseInt(readLine(stdin))
    except:
        result = -1

var first = get_depth()
var second = get_depth()
var third = get_depth()
var prev = -1
var current = 0
var counter = 0
echo first, " ", second, " ", third

while third != -1:
    current = first + second + third
    if current > prev and prev != -1:
        echo prev, " ", current, " bigger"
        inc(counter)
    else:
        echo prev, " ", current, " smaller"

    prev = current
    first = second
    second = third
    third = get_depth()
    echo first, " ", second, " ", third
    
echo counter