#!/usr/bin/env python3

import argparse
from collections import OrderedDict
import logging
import sys
import time
from typing import Sequence

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
    info(f"inputs: {inputs}")
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
    return [l.encode() for l in input().split(",") if l]


def sol1(inputs):
    tot = 0
    for seq in inputs:
        h = 0
        for c in seq:
            h = ((h + c) * 17) % 256
        tot += h
    return tot


def focusing(boxes: Sequence[dict[bytes, int]]) -> int:
    tot = 0
    for i, box in enumerate(boxes, 1):
        for j, v in enumerate(box.values(), 1):
            power = i * j * v
            dbg(
                "box %d, element %d: %d, power = %d %d %d = %d",
                i - 1,
                j,
                v,
                i,
                j,
                v,
                power,
            )
            tot += power
    return tot


def sol2(inputs: list[bytes]):
    boxes = [OrderedDict() for _ in range(256)]
    for seq in inputs:
        h = 0
        dbg("seq: %s", seq)
        for i, c in enumerate(seq):
            if c == ord("="):
                dbg("found =")
                break
            if c == ord("-"):
                dbg("found -")
                continue
            h = ((h + c) * 17) % 256
        else:
            # found -
            val = boxes[h].pop(seq[:-1], None)
            dbg("no = found, hash = %d, key = %s, popped %s", h, seq[:-1], val)
            continue
        # found =
        dbg(
            "found =, hash = %d, key = %s, inserting val = %s", h, seq[:i], seq[i + 1 :]
        )
        boxes[h][seq[:i]] = int(seq[i + 1 :])
    info("boxes: %s", boxes)
    res = focusing(boxes)
    return res


if __name__ == "__main__":
    main()
