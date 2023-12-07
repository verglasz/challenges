#!/usr/bin/env python

import re


def input_lines():
    while True:
        try:
            yield input()
        except EOFError:
            break


def part_1():
    tot = 0
    for l in input_lines():
        nums = [x for x in l if x.isdigit()]
        if not nums:
            continue
        fst = nums[0]
        last = nums[-1]
        n = fst + last
        tot += int(n)
    print(tot)


values = {
    "one": "1",
    "1": "1",
    "two": "2",
    "2": "2",
    "three": "3",
    "3": "3",
    "four": "4",
    "4": "4",
    "five": "5",
    "5": "5",
    "six": "6",
    "6": "6",
    "seven": "7",
    "7": "7",
    "eight": "8",
    "8": "8",
    "nine": "9",
    "9": "9",
    "zero": "0",
    "0": "0",
}


def part_2():
    tot = 0
    findnums = re.compile(r"one|two|three|four|five|six|seven|eight|nine|\d")
    for l in input_lines():
        nums = list(findnums.finditer(l))
        if not nums:
            continue
        fst = values[nums[0].group()]
        last = nums[-1]
        while 1:
            pastlast = last.start() + 1
            x = findnums.search(l, pastlast)
            if x is None:
                break
            last = x
        last = values[last.group()]
        n = fst + last
        tot += int(n)
    print(tot)


if __name__ == "__main__":
    # part_1()
    part_2()
