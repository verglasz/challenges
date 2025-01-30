#!/usr/bin/env python3

import argparse
import logging
import sys
import time

import numpy as np
from numba import njit
from numpy._typing import _Shape

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
    info("inputs:\n%s", inputs)
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
    grid = [list(l.encode()) for l in input_lines() if l]
    return np.array(grid, dtype=np.uint8)

Point = tuple[int,int]

@njit
def addp(pos: Point, dir: Point) -> Point:
    return (pos[0]+dir[0], pos[1]+dir[1])

@njit
def inbounds(pos: Point, shape: Point) -> bool:
    return 0 <= pos[0] < shape[0] and 0 <= pos[1] < shape[1]

State = np.ndarray[_Shape, np.dtype[np.int8]]

@njit
def sol1_jit(grid: np.ndarray, startpos=(0,0), startdir=(0,1))->int:
    state: State = np.zeros((*grid.shape, 4), dtype=np.int8)
    beams: list[tuple[Point, Point]] = [(startpos, startdir)]
    while beams:
        new_beams = []
        for pos,dir in beams:
            x,y = pos
            dir_idx = (x+1) + (y+1)//2
            if state[x,y,dir_idx]:
                continue
            else:
                state[x,y,dir_idx] = 1
            if grid[pos] == 0x5c:
                new_dirs = [(y, x)]
            elif grid[pos] == 0x2f:
                new_dirs = [(-y, -x)]
            elif grid[pos] == 0x2d and y == 0:
                new_dirs = [(0, -1), (0, 1)]
            elif grid[pos] == 0x7c and x == 0:
                new_dirs = [(-1, 0), (1, 0)]
            else:
                new_dirs = [dir]
            for new_dir in new_dirs:
                p = addp(pos, new_dir)
                if inbounds(p,grid.shape):
                    new_beams.append((p, new_dir))
        beams = new_beams
    return np.sum(state)

def sol1(grid: np.ndarray, startpos=(0,0), startdir=(0,1)):
    state = {}
    beams = [(startpos, startdir)]
    while beams:
        new_beams = []
        for pos,dir in beams:
            if pos in state and dir in state[pos]:
                continue
            state.setdefault(pos, set()).add(dir)
            match grid[pos], dir:
                case 0x5c, (n,m): # 0x5c = ord(\)
                    # horizontal beam reflected to vertical and vice versa
                    # the signs match
                    new_dirs = [(m, n)]
                case 0x2f, (m,n): # 0x2f = ord(/)
                    # vertical beam reflected to horizontal and vice versa
                    # the signs swap
                    new_dirs = [(-n, -m)]
                case 0x2d, (_,0): # 0x2d = ord(-)
                    # vertical beam split
                    new_dirs = [(0, -1), (0, 1)]
                case 0x7c, (0,_): # 0x7c = ord(|)
                    # horizontal beam split
                    new_dirs = [(-1, 0), (1, 0)]
                case _, d: # 0x2e = ord(.), or unsplit
                    # empty space
                    new_dirs = [d]
                    # raise ValueError(f"invalid grid value {chr(grid[pos])} in {pos} with dir {dir}")
            for new_dir in new_dirs:
                p = addp(pos, new_dir)
                if inbounds(p,grid.shape):
                    new_beams.append((p, new_dir))
        beams = new_beams
    return len(state)


@njit
def sol2(grid: np.ndarray):
    h,w =  grid.shape
    res = 0
    for i in range(h):
        startpos = (i,0)
        res = max(res, sol1_jit(grid, startpos, (0,1)))
        startpos = (i,w-1)
        res = max(res, sol1_jit(grid, startpos, (0,-1)))
    for j in range(w):
        startpos = (0,j)
        res = max(res, sol1_jit(grid, startpos, (1,0)))
        startpos = (h-1,j)
        res = max(res, sol1_jit(grid, startpos, (-1,0)))
    return res


if __name__ == "__main__":
    main()

