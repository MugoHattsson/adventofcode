import std/strutils
import std/sequtils


proc valid_password(min: int, max: int, key: char, pw: string): bool =
    var count = count(pw, key)
    return count >= min and count <= max

proc valid_password2(min: int, max: int, key: char, pw: string): bool =
    return pw[min-1] == key xor pw[max-1] == key

var line: string
var elements: seq[string]
var counter: int = 0

while readLine(stdin, line):
    elements = split(line, " ")
    var limits: seq[int] = map(split(elements[0], "-"), parseInt)
    var key: char = elements[1][0]
    var password: string = elements[2]

    if valid_password2(limits[0], limits[1], key, password): inc(counter)
    
echo counter