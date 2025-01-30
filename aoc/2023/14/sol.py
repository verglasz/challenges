#!/usr/bin/env python3

import argparse
import logging
import sys
import time
from typing import Any
import numpy as np
from numba import njit

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
    if 1 in args.problem:
        start = time.time()
        print(sol1(inputs))
        end = time.time()
        eprint(f"sol 1 took {(end-start)*1000:.2f}ms")
    if 2 in args.problem:
        # warmup
        eprint(f"warmup numba")
        sol2_inplace(inputs.copy(), 10)
        eprint(f"warmup done")
        start = time.time()
        print(sol2_inplace(inputs, 1_000_000_000))
        end = time.time()
        eprint(f"sol 2 took {(end-start)*1000:.2f}ms")


def parse():
    lines = [list(l.encode()) for l in input_lines() if l]
    a = np.array(lines, dtype=np.int8)
    h, w = a.shape
    for i in range(h):
        for j in range(w):
            if a[i, j] == ord("O"):
                a[i, j] = ROUND
            elif a[i, j] == ord("#"):
                a[i, j] = SQUARE
            elif a[i, j] == ord("."):
                a[i, j] = EMPTY
            else:
                raise ValueError(f"invalid input {a[i, j]} at {i},{j}")
    info("%s\n%s", a.shape, a)
    return a


ROUND = 0  # ord("O")
SQUARE = 1  # ord("#")
EMPTY = -1  # ord(".")


def sol1(inputs: np.ndarray[Any, np.dtype[np.int8]]):
    rocks = inputs.T
    h, w = rocks.shape
    tot = 0
    for row in rocks:
        row: np.ndarray[Any, np.dtype[np.int8]]
        load = w
        for j, rock in enumerate(row):
            if rock == ROUND:
                tot += load
                load -= 1
            elif rock == SQUARE:
                load = w - j - 1
    return tot


def tilt(rocks: np.ndarray[Any, np.dtype[np.int8]], direction: int):
    if direction == 0:
        rocks = rocks.T
    elif direction == 1:
        rocks = rocks[::, ::]
    elif direction == 2:
        rocks = rocks[::-1, ::].T
    elif direction == 3:
        rocks = rocks[::, ::-1]
    h, w = rocks.shape
    mapping = {}
    for i in range(h):
        pos = 0
        for j in range(w):
            rock = rocks[i, j]
            if rock == ROUND:
                # mapping[i, j] = (i, pos)
                rocks[i, j] = EMPTY
                rocks[i, pos] = ROUND
                pos += 1
            elif rock == SQUARE:
                pos = j + 1
    if direction == 0:
        rocks = rocks.T
        # mapping = {(y, x): (j, i) for (x, y), (i, j) in mapping.items()}
    elif direction == 1:
        rocks = rocks
    elif direction == 2:
        rocks = rocks.T[::-1, ::]
        # new = {}
        # for (x, y), (i, j) in mapping.items():
        #     dbg("turning mapping %s -> %s", (x, y), (i, j))
        #     ny = w - y - 1
        #     nj = w - j - 1
        #     dbg("into %s -> %s", (ny, x), (nj, i))
        #     new[ny, x] = (nj, i)
        # mapping = {
        #     (h - y - 1, h - x - 1): (j, h - i - 1) for (x, y), (i, j) in mapping.items()
        # }
        # mapping = new
    elif direction == 3:
        rocks = rocks[::, ::-1]
        # mapping = {(x, w - y - 1): (i, w - j - 1) for (x, y), (i, j) in mapping.items()}
    return rocks, mapping


@njit
def tilt_inplace(rocks: np.ndarray[Any, np.dtype[np.int8]], direction: int):
    h, w = rocks.shape
    if direction == 0:
        for j in range(w):
            pos = 0
            for i in range(h):
                rock = rocks[i, j]
                if rock == ROUND:
                    rocks[i, j] = EMPTY
                    rocks[pos, j] = ROUND
                    pos += 1
                elif rock == SQUARE:
                    pos = i + 1
    elif direction == 1:
        for i in range(h):
            pos = 0
            for j in range(w):
                rock = rocks[i, j]
                if rock == ROUND:
                    rocks[i, j] = EMPTY
                    rocks[i, pos] = ROUND
                    pos += 1
                elif rock == SQUARE:
                    pos = j + 1
    elif direction == 2:
        for j in range(w):
            pos = h - 1
            for i in range(h - 1, -1, -1):
                rock = rocks[i, j]
                if rock == ROUND:
                    rocks[i, j] = EMPTY
                    rocks[pos, j] = ROUND
                    pos -= 1
                elif rock == SQUARE:
                    pos = i - 1
    elif direction == 3:
        for i in range(h):
            pos = w - 1
            for j in range(w - 1, -1, -1):
                rock = rocks[i, j]
                if rock == ROUND:
                    rocks[i, j] = EMPTY
                    rocks[i, pos] = ROUND
                    pos -= 1
                elif rock == SQUARE:
                    pos = j - 1


def cycle(rocks: np.ndarray[Any, np.dtype[np.int8]]):
    rocks, mapping = tilt(rocks, 0)
    # dbg("after tilt to %d :\n%s\n%s", 0, rocks, mapping)
    base_mapping = mapping
    for i in range(1, 4):
        rocks, mapping = tilt(rocks, i)
        # dbg("after tilt to %d :\n%s\n%s", i, rocks, mapping)
        # for k, v in base_mapping.items():
        #     base_mapping[k] = mapping[v]
    return rocks, base_mapping


@njit
def cycle_inplace(rocks: np.ndarray[Any, np.dtype[np.int8]]):
    tilt_inplace(rocks, 0)
    tilt_inplace(rocks, 1)
    tilt_inplace(rocks, 2)
    tilt_inplace(rocks, 3)


# @njit
def sol2_inplace(inputs: np.ndarray[Any, np.dtype[np.int8]], cycles):
    rocks = inputs
    seen = {}
    byts = rocks.tobytes()
    for i in range(cycles):
        seen[byts] = i
        cycle_inplace(rocks)
        byts = rocks.tobytes()
        # dbg("after %d cycles:\n%s", i + 1, rocks)
        if byts in seen:
            start = seen[byts]
            length = i + 1 - start
            # warn("found rock cycle at %d, start %d len %d", i + 1, start, length)
            break
    else:
        # err("did not find rock cycle")
        return north_load(rocks)
    left = cycles - start
    rem = left % length
    for i in range(rem):
        cycle_inplace(rocks)
    return north_load(rocks)


@njit
def north_load(rocks: np.ndarray[Any, np.dtype[np.int8]]) -> int:
    h, w = rocks.shape
    tot = 0
    for i in range(h):
        load = h - i
        for j in range(w):
            if rocks[i, j] == ROUND:
                tot += load
    return tot


def sol2(inputs: np.ndarray[Any, np.dtype[np.int8]], cycles=2):
    rocks = inputs
    seen = {}

    seen_rocks: dict[bytes, int] = {rocks.tobytes(): 0}
    past = [rocks]

    # seen = {k: {k: 0, v: 1} for k, v in mapping.items()}
    # total_mapping = mapping
    # found_cycles = {}
    # dbg("after %d cycles:\n%s\n%s", 1, rocks, mapping)
    for i in range(1, cycles + 1):
        rocks, mapping = cycle(rocks)
        byts = rocks.tobytes()
        dbg("after %d cycles:\n%s", i + 1, rocks)
        if byts in seen_rocks:
            start = seen_rocks[byts]
            length = i - start
            warn("found rock cycle at %d, start %d len %d", i, start, length)
            break
        seen_rocks[byts] = i
        past.append(rocks)
        # for k, v in total_mapping.items():
        #     new = mapping[v]
        #     if new in seen[k]:
        #         start = seen[k][new]
        #         info(
        #             "found cycle for %s at %d, start %d len %d", k, i, start, i - start
        #         )
        #         found_cycles[k] = (start, i - start)
        #     total_mapping[k] = new
    else:
        err("did not find rock cycle")
        return None
    left = cycles - start
    rem = left % length
    warn("left %d rem %d so equal to %d", left, rem, rem + start)

    end = past[start + rem]
    return north_load(end)


if __name__ == "__main__":
    main()
