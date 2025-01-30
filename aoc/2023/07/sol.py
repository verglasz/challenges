#!/usr/bin/env python3

from collections import namedtuple
import re
import sys
from types import SimpleNamespace


eprint = (
    lambda *args, **kwargs: print(*args, file=sys.stderr, **kwargs)
    if kwargs.pop("skip", True)
    else None
)


def input_lines():
    while True:
        try:
            yield input()
        except EOFError:
            break


class Strength:
    voak = re.compile(r"(.)\1{4}")
    foak = re.compile(r"(.)\1{3}")
    full = re.compile(r"(.)\1\1(.)\2|(.)\3(.)\4\4")
    toak = re.compile(r"(.)\1\1")
    twop = re.compile(r"(.)\1(.)\2|(.)\3.(.)\4")
    pair = re.compile(r"(.)\1")


class Jrength:
    voak = re.compile(r"(.)(\1|J){4}")
    foak = re.compile(r"(.)(\1|J){3}")
    full = re.compile(r"(.)(\1|J){2}(.)(\2|J)|(.)(\3|J)(.)(\4|J){2}")
    toak = re.compile(r"(.)")
    twop = re.compile(r"(.)\1(.)\2|(.)\3.(.)\4")
    pair = re.compile(r"(.)\1")


def parse_jand_old(hand: str):
    eprint("parsing", hand)
    assert len(hand) == 5
    cards = list(set(hand))
    eprint("cards", cards, skip="J" in hand)
    handval = tuple(jal(c) for c in hand)
    score = 0
    if hand == "JJJJJ":
        eprint("strength", 6, "tie", handval)
        return (6, handval)
    for i, c in enumerate(cards):
        if c == "J":
            continue
        count = len(re.findall(f"{c}|J", hand))
        if count == 5:
            score = max(score, 6)
        elif count == 4:
            score = max(score, 5)
        elif count == 3:
            for c2 in cards:
                if c2 == "J" or c2 == c:
                    continue
                othercount = hand.count(c2)
                if othercount == 2:
                    score = max(score, 4)
            score = max(score, 3)
        elif count == 2:
            for c2 in cards:
                if c2 == "J" or c2 == c:
                    continue
                othercount = hand.count(c2)
                if othercount == 3:
                    score = max(score, 4)
                elif othercount == 2:
                    score = max(score, 2)
            score = max(score, 1)
    eprint("strength", score, "tie", handval, skip="J" in hand)
    return (score, handval)


def parse_jand(hand: str, wild="J"):
    eprint("parsing", hand)
    assert len(hand) == 5
    cards = list(set(hand))
    eprint("cards", cards, skip=wild in hand)
    handval = tuple(jal(c, wild) for c in hand)
    jokers = hand.count(wild)
    if jokers == 5:
        eprint("strength", 6, "tie", handval)
        return (6, handval)
    score = 0
    counts = sorted(hand.count(c) for c in cards if c != wild)[::-1]
    match counts[0] + jokers:
        case 5:
            score = 6
        case 4:
            score = 5
        case 3:
            if counts[1] == 2:
                score = 4
            else:
                score = 3
        case 2:
            if counts[1] == 2:
                score = 2
            else:
                score = 1
        case _:
            score = 0

    eprint("strength", score, "tie", handval, skip="J" in hand)
    return (score, handval)


def parse_hand(hand: str):
    eprint("parsing", hand)
    assert len(hand) == 5
    cs = list(hand)
    cards = "".join(sorted(cs))
    eprint("cards", cs, cards)
    if Strength.voak.search(cards):
        s = 6
    elif Strength.foak.search(cards):
        s = 5
    elif Strength.full.search(cards):
        s = 4
    elif Strength.toak.search(cards):
        s = 3
    elif Strength.twop.search(cards):
        s = 2
    elif Strength.pair.search(cards):
        s = 1
    else:
        assert len(set(hand)) == 5
        s = 0
    eprint("strength", s)
    return (s, tuple(val(c) for c in cs))


def jal(c, wild="J"):
    if c == wild:
        return -1
    if c == "A":
        return 14
    if c == "K":
        return 13
    if c == "Q":
        return 12
    if c == "J":
        return 11
    if c == "T":
        return 10
    return int(c)


def val(c):
    if c == "A":
        return 14
    if c == "K":
        return 13
    if c == "Q":
        return 12
    if c == "J":
        return 11
    if c == "T":
        return 10
    return int(c)


def sol1(inputs):
    inps = []
    for line in inputs:
        if not line:
            continue
        hand, bid = line.split()
        inps.append((int(bid), parse_hand(hand)))
    inps.sort(key=lambda x: x[1])
    tot = 0
    for rank, (bid, hand) in enumerate(inps, 1):
        tot += bid * rank
    return tot


def sol2(inputs, wild):
    inps = []
    for line in inputs:
        if not line:
            continue
        hand, bid = line.split()
        inps.append((int(bid), parse_jand(hand, wild)))
    inps.sort(key=lambda x: x[1])
    tot = 0
    for rank, (bid, hand) in enumerate(inps, 1):
        tot += bid * rank
    return tot


def main():
    inputs = [l for l in input_lines() if l]
    print(sol2(inputs, "\0"))
    print(sol2(inputs, "J"))


if __name__ == "__main__":
    main()
