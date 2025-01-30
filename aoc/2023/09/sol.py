#!/usr/bin/env python3

import argparse
import logging
import sys

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
    inputs = [[int(i) for i in l.split()] for l in input_lines() if l]
    # print(sol1(inputs))
    print(sol2(inputs))


def full_diffs(l):
    lastdiff = np.asarray(l)
    diffs = []
    while not np.all(lastdiff == 0):
        diffs.append(lastdiff)
        lastdiff = np.diff(lastdiff)
    return diffs


def sol1(inputs: list[list[int]]):
    res = 0
    for l in inputs:
        diffs = full_diffs(l)
        extrapolated = 0
        for d in diffs[::-1]:
            extrapolated += d[-1]
        res += extrapolated
    return res


def sol2(inputs):
    res = 0
    for l in inputs:
        diffs = full_diffs(l)
        extrapolated = 0
        for d in diffs[::-1]:
            extrapolated = d[0] - extrapolated
        res += extrapolated
    return res


if __name__ == "__main__":
    main()
