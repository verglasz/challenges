#!/usr/bin/env python3

import argparse
import logging
import sys
import time
from typing import Sequence
import numpy as np

eprint = lambda *args, **kwargs: print(*args, file=sys.stderr, **kwargs)

logging.basicConfig(level=logging.DEBUG)
logger = logging.getLogger(__name__)

dbg, info, warn, err = logger.debug, logger.info, logger.warning, logger.error


def input_lines():
    while True:
        try:
            yield input()
        except EOFError:
            break


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "problem",
        nargs="*",
        type=int,
        choices=[1, 2],
        default=[1, 2],
    )
    parser.add_argument("-v", "--verbose", action="count", default=0)
    parser.add_argument("-q", "--quiet", action="count", default=0)
    args = parser.parse_args()
    if args.verbose >= 1:
        logger.setLevel(logging.DEBUG)
    elif args.quiet >= 2:
        logger.setLevel(logging.ERROR)
    elif args.quiet >= 1:
        logger.setLevel(logging.WARNING)
    else:
        logger.setLevel(logging.INFO)
    inputs = parse()
    if 1 in args.problem:
        start = time.time()
        print(sol1(inputs))
        end = time.time()
        eprint(f"sol 1 took {(end-start)*1000:.2f}ms")
    if 2 in args.problem:
        start = time.time()
        print(sol2(inputs))
        end = time.time()
        eprint(f"sol 2 took {(end-start)*1000:.2f}ms")


def parse():
    cases = []
    case = []
    for line in input_lines():
        if not line:
            cases.append(np.array(case, dtype=np.int8))
            case = []
        else:
            case.append([int(c == "#") for c in line])
    if case:
        cases.append(np.array(case, dtype=np.int8))
    return cases


def palindromes(l: Sequence):
    for i in range(1, len(l) - 1):
        width = min(i, len(l) - i)
        start = 0 if width == i else i - width
        end = i + width if width == i else len(l)
        left = l[start:i]
        right = l[end - 1 : i - 1 : -1]
        pal = left == right
        if pal:
            yield i


def mirror(case: np.ndarray, skip=(0, 0)) -> tuple[int, int]:
    h, w = case.shape
    # info(f"{h=},{w=}")
    row = 0
    for i in range(1, h):
        if i == skip[0]:
            continue
        width = min(i, h - i)
        start = 0 if width == i else i - width
        end = i + width if width == i else h
        dbg(f"{i=},{width=},{start=},{end=}")
        left = case[start:i]
        right = case[end - 1 : i - 1 : -1]
        dbg("left\n%s\nright\n%s", left, right)
        pal = np.all(left == right)
        if pal:
            info(f"palindrome at row {i=}")
            return i, 0
    for j in range(1, w):
        if j == skip[1]:
            continue
        width = min(j, w - j)
        start = 0 if width == j else j - width
        end = j + width if width == j else w
        left = case[:, start:j]
        right = case[:, end - 1 : j - 1 : -1]
        pal = np.all(left == right)
        if pal:
            info(f"palindrome at col {j=}")
            return 0, j
    info("no palindrome found")
    dbg(" for \n%s", case)
    return 0, 0


def sol1(inputs):
    tot = 0
    warn(f"{len(inputs)} cases")
    for i, case in enumerate(inputs):
        row, col = mirror(case)
        info(f"case {i:4} mirror with {row=},{col=}")
        tot += row * 100 + col
    return tot


def sol2(inputs):
    tot = 0
    warn(f"{len(inputs)} cases")
    for n, case in enumerate(inputs):
        base = mirror(case)
        h, w = case.shape
        for i in range(h):
            for j in range(w):
                new = np.copy(case)
                new[i, j] = 1 - new[i, j]
                info(f"flipped {i=},{j=}")
                result = mirror(new, skip=base)
                if result != (0, 0) and result != base:
                    break
            else:
                continue
            break
        else:
            err("no mirror found for case %d", n)
            raise RuntimeError("no mirror found")
        warn(f"case {n:4} mirror with {result}")
        row, col = result
        tot += row * 100 + col
    return tot


if __name__ == "__main__":
    main()
