import std/strutils
import std/sequtils

type
    Field = enum
        byr, iyr, eyr, hgt, hcl, ecl, pid, cid

proc get_passport_lines(): string =
    var line: string
    discard readLine(stdin, line)
    while line != "":
        result &= line & " "
        discard readLine(stdin, line)

proc valid_field(field: string): bool =
    try:
        discard parseEnum[Field](field)
        return true
    except ValueError:
        return false


proc valid_passport(fields: seq[string]): bool =
    var valids: seq[bool] = repeat(false, 8)
    valids[7] = true

    for field in fields:
        if valid_field(field):
            var e = parseEnum[Field](field)
            valids[ord(e)] = true

    return all(valids, proc(x: bool): bool = x == true)

var passport: string = get_passport_lines()
var counter = 0
var fields: seq[string]

while passport != "":
    fields = map(split(strip(passport), " "), proc(x: string): string = x[0..2])
    if valid_passport(fields):
        inc(counter)
    passport = get_passport_lines()

echo counter
