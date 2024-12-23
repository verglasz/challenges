#!/usr/bin/env python3

from collections import namedtuple
import sys
from types import SimpleNamespace
import logging
import math

eprint = lambda *args, **kwargs: print(*args, file=sys.stderr, **kwargs)

dbg, info, warn = logging.debug, logging.info, logging.warning


def input_lines():
    while True:
        try:
            yield input()
        except EOFError:
            break


def getp(l):
    a = l.split("=")[1].strip()
    return tuple(x.strip() for x in a[1:-1].split(","))


def inputs():
    lrlist = input()
    dbg(lrlist)
    lr = [0 if c == "L" else 1 for c in lrlist]
    dbg(lr)
    assert not input()
    lines = [l for l in input_lines() if l]
    dirs = {l[:3]: getp(l) for l in lines}
    dbg(dirs)
    return lr, dirs


def sol1(lr, dirs):
    pos = "AAA"
    count = 0
    while pos != "ZZZ":
        lrpos = lr[count % len(lr)]
        dbg(f"{pos} {lrpos} -> {dirs[pos][lrpos]} (count={count})")
        pos = dirs[pos][lrpos]
        count += 1
    return count


def allz(pos: set):
    return all(p.endswith("Z") for p in pos)


def sol2_bad(lr: list, dirs: dict):
    posset = {k for k in dirs.keys() if k.endswith("A")}
    eprint(f"start {posset} ({len(posset)=})")
    count = 0
    to_z_counts = {}
    while not allz(posset):
        lrpos = lr[count % len(lr)]
        new = {dirs[pos][lrpos] for pos in posset}
        eprint(f"size {len(posset):5} {lrpos} -> {len(new):5} (count={count})")
        eprint(f"\t{posset} -> {new}")
        posset = new
        count += 1
    return count


def sol2_dumb(lr: list, dirs: dict):
    # this works by "pure luck" (aka because the input is kind enough to be one where this works...)
    startpos = [k for k in dirs.keys() if k.endswith("A")]
    warn(f"start {startpos} ({len(startpos)=})")
    mods = []
    for pos in startpos:
        n, p = count_to_z(pos, dirs, lr)
        mods.append(n)
    warn(f"{mods=}")
    return lcm(mods)


def sol2_mess(lr: list, dirs: dict):
    startpos = [k for k in dirs.keys() if k.endswith("A")]
    warn(f"start {startpos} ({len(startpos)=})")
    loops = {}
    to_z_counts = {}
    warn(f"looking at start post loops and z counts")
    for p in startpos:
        logging.info(f"start {p}")
        loops[p] = count_loop(p, dirs, lr)
        logging.info(f"found loop {loops[p]}")
        to_z_counts[p] = count_to_z(p, dirs, lr)
        logging.info(f"found count to z {to_z_counts[p]}")
    warn(f"{loops=}")
    warn(f"looking at z loops")
    z_loops = {}
    for k, v in to_z_counts.items():
        n, p = v
        z_loops[p] = count_zloop(p, dirs, lr, start=n)
    warn(f"{z_loops=}")
    warn(f"looking at z counts for loops")
    for k, v in loops.items():
        n, p, _ = v
        logging.info(f"{k=} {v=} {p=}")
        to_z_counts[p] = count_to_z(p, dirs, lr, start=n)

    logging.warning(f"{to_z_counts=}")
    # return lcm(to_z_counts)


def gcd(a, b):
    if a < b:
        a, b = b, a
    while b:
        a, b = b, a % b
    return a


def lcm(nums):
    res = 1
    gcds = []
    for n in nums:
        mult = gcd(res, n)
        res *= n // mult
        if mult != 1:
            gcds.append(mult)
    info(f"{gcds=}")
    return res


def count_to_z(pos: str, dirs: dict, lr: list, start=0):
    count = start
    while not pos.endswith("Z"):
        lrpos = lr[count % len(lr)]
        dbg(f"{pos} {lrpos} -> {dirs[pos][lrpos]} ({count=})")
        pos = dirs[pos][lrpos]
        count += 1
    return count, pos


def count_loop(pos: str, dirs: dict, lr: list, start=0):
    count = start
    seen = {}
    while pos not in seen:
        seen[pos] = count
        lrpos = lr[count % len(lr)]
        dbg(f"{pos} {lrpos} -> {dirs[pos][lrpos]} ({count=})")
        pos = dirs[pos][lrpos]
        count += 1
    return seen[pos], pos, count - seen[pos]


def count_zloop(pos: str, dirs: dict, lr: list, start=0):
    count = start
    seen = {}
    while not pos.endswith("Z"):
        if pos in seen:
            return count - seen[pos], None
        seen[pos] = count
        lrpos = lr[count % len(lr)]
        dbg(f"{pos} {lrpos} -> {dirs[pos][lrpos]} ({count=})")
        pos = dirs[pos][lrpos]
        count += 1
    return count, pos


def modular_sol(modeqs: dict[int, int]) -> int | None:
    mods = sorted(m for m in modeqs)
    lean_mods = []
    for m in mods:
        for n in lean_mods:
            g = gcd(m, n)


def sol2(lr: list, dirs: dict):
    # for a full solution, we need to do something fairly complicated...
    # we need to find the loops (of length n_i, starting at s_i) for
    # each start position i
    # and the index inside the loop of every Z-ending position (z_i,j),
    # then we need to find the solutions to the system of equations
    # { x ≡ a_i,{j_i} mod n_i }_i
    # for arbitrary choices of j_i (ie, for any starting position i we can end in
    # any of its Z-ending positions j_i, and we need to do so for all starting position i)
    # where a_i,j = (z_i,j + s_i) % n_i
    # (ie, a_i is the index of the Z-ending position in the loop
    # including the running up to the loop)
    # once we have the solutions (some of these systems will admit no solution
    # due to the hypotheses of the Chinese Remainder Theorem not being met),
    # we need to find the smallest positive solution x.
    # ah, and we also need to check if we hit a solution _before_ the various positions
    # start looping, but that we can do with a simple linear search as we look for the loop
    # starts anyway
    #
    # this has complexity O(p^m * CRT(l))
    # (where p is the number of z-solutions per position, m is the number of positions,
    # l is the length of the loops and CRT is the complexity of finding the solution
    # to the modular equations)
    # so it's actually pretty bad if p is larger than 3-4 and m is larger than 10.
    #
    # a different approach for a full solution (thx Tove) would be to find the loops
    # per position, then step by the loop length for each position's z-solution
    # until we find that the paths for each position are in sync,
    # if i'm not mistaken this has complexity O(N/l * p*m)
    # where N is the solution and l,p,m are as before
    # so if N is not too large and l is not too small, this ends up being better

    startpos = [k for k in dirs.keys() if k.endswith("A")]
    warn(f"start {startpos} ({len(startpos)=})")
    to_z_counts = {}
    warn(f"looking at start post, z counts:")
    for pos in startpos:
        logging.info(f"start {pos}")
        n, p = count_to_z(pos, dirs, lr)
        to_z_counts[pos] = n, p
        logging.info(f"found count to z {to_z_counts[pos]}")
    warn(f"{to_z_counts=}")
    warn(f"looking at z loops")
    z_loops = {}
    for k, v in to_z_counts.items():
        n, p = v
        z_loops[p] = count_zloop(p, dirs, lr, start=n)
    warn(f"{z_loops=}")
    modeq = {}
    for start, p in to_z_counts.values():
        mod, _ = z_loops[p]
        rem = start % mod
        if mod in modeq:
            if modeq[mod] != rem:
                return None
        else:
            modeq[mod] = rem
    warn(f"{modeq=}")
    if all(v == 0 for v in modeq.values()):
        return lcm(k for k in modeq)
    return None


def main():
    logging.basicConfig(level=logging.INFO)
    if len(sys.argv) < 2:
        logging.basicConfig(level=logging.DEBUG)
    lr, dirs = inputs()
    # print(sol1(lr, dirs))
    print(sol2(lr, dirs))


main()
