#!/usr/bin/env python3

import argparse
import logging
import sys
import time

import numpy as np

eprint = lambda *args, **kwargs: print(*args, file=sys.stderr, **kwargs)

logging.basicConfig(level=logging.DEBUG)
logger = logging.getLogger(__name__)

dbg, info, warn, err = logger.debug, logger.info, logger.warning, logger.error


def eprint(*args, **kwargs):
    print(*args, file=sys.stderr, **kwargs)


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
        type=lambda x: int(x) if x is not None else [1, 2],
        choices=[1, 2],
        default=None,
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
        print(sol(inputs, 2))
        end = time.time()
        eprint(f"sol 1 took {(end-start)*1000:.2f}ms")
    if 2 in args.problem:
        start = time.time()
        print(sol(inputs, 1_000_000))
        end = time.time()
        eprint(f"sol 2 took {(end-start)*1000:.3f}ms")


def parse():
    return np.array([[int(c == "#") for c in l] for l in input_lines() if l])


def manhattan(a, b):
    return abs(a[0] - b[0]) + abs(a[1] - b[1])


def expanded_manhattan(a, b, expand_row, expand_col, scale=1):
    extra_row = [i for i in expand_row if a[0] < i < b[0] or b[0] < i < a[0]]
    extra_col = [j for j in expand_col if a[1] < j < b[1] or b[1] < j < a[1]]
    dbg(
        f"{a=} {b=} {extra_row=} (len {len(extra_row)}) {extra_col=} (len {len(extra_col)})"
    )
    extra_rows = len(extra_row)
    extra_cols = len(extra_col)
    return abs(a[0] - b[0]) + abs(a[1] - b[1]) + (scale - 1) * (extra_rows + extra_cols)


def sol1(universe):
    info(f"input:  {universe.shape}")
    dbg(f"\n{universe!r}")
    h, w = universe.shape
    expand_row = []
    for i in range(h):
        if not any(universe[i, :]):
            expand_row.append(i)
    expand_col = []
    for j in range(w):
        if not any(universe[:, j]):
            expand_col.append(j)
    universe = np.insert(universe, expand_row, np.zeros(w, dtype=bool), axis=0)
    h, w = universe.shape
    universe = np.insert(
        universe, expand_col, np.zeros((h, len(expand_col)), dtype=bool), axis=1
    )
    h, w = universe.shape
    info(f"expanded shape: {universe.shape}")
    dbg(f"\n{universe!r}")
    galaxies = []
    total = 0
    for i in range(h):
        for j in range(w):
            if universe[i, j]:
                for g in galaxies:
                    total += manhattan((i, j), g)
                galaxies.append((i, j))
    info(f"processed {len(galaxies)} galaxies")
    return total


def sol(universe, scale):
    info(f"input:  {universe.shape}")
    # info(f"\n{universe!r}")
    h, w = universe.shape
    empty_rows = ~np.any(universe, axis=1)
    empty_cols = ~np.any(universe, axis=0)
    # info(f"\n{empty_rows=}\n{empty_cols=}")
    shift = scale - 1
    row_shift = np.cumsum(empty_rows, dtype=np.uint32) * shift
    col_shift = np.cumsum(empty_cols, dtype=np.uint32) * shift
    # info(f"\n{row_shift=}\n{col_shift=}")
    galaxies = []
    total = 0
    for i in range(h):
        for j in range(w):
            if universe[i, j]:
                pos = (i + row_shift[i], j + col_shift[j])
                for g in galaxies:
                    total += manhattan(pos, g)
                galaxies.append(pos)
    info(f"processed {len(galaxies)} galaxies")
    return total


def sol2(universe, scale):
    info(f"input:  {universe.shape}")
    dbg(f"\n{universe!r}")
    h, w = universe.shape
    expand_row = []
    for i in range(h):
        if not any(universe[i, :]):
            expand_row.append(i)
    expand_col = []
    for j in range(w):
        if not any(universe[:, j]):
            expand_col.append(j)
    dbg(f"\n{expand_row=}\n{expand_col=}")
    galaxies = []
    total = 0
    for i in range(h):
        for j in range(w):
            if universe[i, j]:
                for g in galaxies:
                    total += expanded_manhattan(
                        (i, j), g, expand_row, expand_col, scale
                    )
                galaxies.append((i, j))
    info(f"processed {len(galaxies)} galaxies")
    return total


if __name__ == "__main__":
    main()
