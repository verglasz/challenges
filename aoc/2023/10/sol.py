#!/usr/bin/env python3

import argparse
import logging
import sys

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
        print(sol1(inputs))
    if 2 in args.problem:
        print(sol2(inputs))


def parse():
    return [list(l) for l in input_lines() if l]


def maybe_neighbours(i, j, max_i, max_j):
    # clockwise from top
    if i > 0:
        yield i - 1, j
    else:
        yield None
    if j < max_j - 1:
        yield i, j + 1
    else:
        yield None
    if i < max_i - 1:
        yield i + 1, j
    else:
        yield None
    if j > 0:
        yield i, j - 1
    else:
        yield None


def neighbours(i, j, max_i, max_j):
    # clockwise from top
    if i > 0:
        yield i - 1, j
    if j < max_j - 1:
        yield i, j + 1
    if i < max_i - 1:
        yield i + 1, j
    if j > 0:
        yield i, j - 1


def connecting_neighbors(pos: tuple[int, int], grid):
    i, j = pos
    current = grid[i][j]
    for ni, nj in neighbours(i, j, len(grid), len(grid[0])):
        c = grid[ni][nj]
        if ni < i and c in ("|", "7", "F"):
            if current in ("S", "|", "J", "L"):
                yield (ni, nj), 0
        elif nj > j and c in ("-", "J", "7"):
            if current in ("S", "-", "F", "L"):
                yield (ni, nj), 1
        elif ni > i and c in ("|", "J", "L"):
            if current in ("S", "|", "7", "F"):
                yield (ni, nj), 2
        elif nj < j and c in ("-", "F", "L"):
            if current in ("S", "-", "J", "7"):
                yield (ni, nj), 3


def visit_loop(animal: tuple[int, int], grid):
    visited = {animal: 0}
    dist = 1
    first = [p for p, _ in connecting_neighbors(animal, grid)]
    a, b = first
    while a != b:
        visited[a] = dist
        visited[b] = dist
        a = next(p for p, _ in connecting_neighbors(a, grid) if p not in visited)
        b = next(p for p, _ in connecting_neighbors(b, grid) if p not in visited)
        dist += 1
    visited[a] = dist
    return visited, dist


def sol1(inputs):
    animal_pos = next(
        (i, j) for i, l in enumerate(inputs) for j, c in enumerate(l) if c == "S"
    )
    grid = inputs
    dbg("\n" + "\n".join("".join(l) for l in grid))
    visited, dist = visit_loop(animal_pos, grid)
    return dist


PIPES = "-|7JFLS"


def hilight(grid, pos: set) -> str:
    bold_red = "\033[1;31m"
    return "\n".join(
        "".join(
            f"{bold_red if (i, j) in pos else ''}{c}\033[0m" for j, c in enumerate(l)
        )
        for i, l in enumerate(grid)
    )


def get_lr_sides(pos, dir, grid):
    i, j = pos
    start = grid[i][j]
    dbg(f"getting sides of {pos=} from {dir=} ({start})")
    i, j = pos
    n, e, s, w = maybe_neighbours(i, j, len(grid), len(grid[0]))
    dbg(f"{n=} {e=} {s=} {w=}")
    if dir == 0:
        if start == "|":
            left_ns = [w]
            right_ns = [e]
        elif start == "7":
            left_ns = []
            right_ns = [e, n]
        elif start == "F":
            left_ns = [n, w]
            right_ns = []
        else:
            raise ValueError(f"dir 0 but {start=}")
    elif dir == 1:
        if start == "-":
            left_ns = [n]
            right_ns = [s]
        elif start == "7":
            left_ns = [n, e]
            right_ns = []
        elif start == "J":
            left_ns = []
            right_ns = [s, e]
        else:
            raise ValueError(f"dir 1 but {start=}")
    elif dir == 2:
        if start == "|":
            left_ns = [e]
            right_ns = [w]
        elif start == "J":
            left_ns = [s, e]
            right_ns = []
        elif start == "L":
            left_ns = []
            right_ns = [w, s]
        else:
            raise ValueError(f"dir 2 but {start=}")
    elif dir == 3:
        if start == "-":
            left_ns = [s]
            right_ns = [n]
        elif start == "L":
            left_ns = [w, s]
            right_ns = []
        elif start == "F":
            left_ns = []
            right_ns = [n, w]
        else:
            raise ValueError(f"dir 3 but {start=}")
    else:
        raise ValueError("dir must be 0-3")
    return [n for n in left_ns if n is not None], [n for n in right_ns if n is not None]


def flood_fill(
    pos: tuple[int, int],
    dir: int,
    grid,
    flood_left: set[tuple[int, int]],
    flood_right: set[tuple[int, int]],
):
    left_ns, right_ns = get_lr_sides(pos, dir, grid)
    dbg(f"def flooding from {pos} {left_ns=} {right_ns=}")
    # dbg("left neighbours\n" + hilight(grid, left_ns))
    # dbg("right neighbours\n" + hilight(grid, right_ns))
    while left_ns:
        ns = []
        for n in left_ns:
            if grid[n[0]][n[1]] in PIPES:
                continue
            if n in flood_left:
                continue
            flood_left.add(n)
            ns.extend(neighbours(n[0], n[1], len(grid), len(grid[0])))
        left_ns = ns
    while right_ns:
        ns = []
        for n in right_ns:
            if grid[n[0]][n[1]] in PIPES:
                continue
            if n in flood_right:
                continue
            flood_right.add(n)
            ns.extend(neighbours(n[0], n[1], len(grid), len(grid[0])))
        right_ns = ns


def sol2(inputs):
    animal_pos = next(
        (i, j) for i, l in enumerate(inputs) for j, c in enumerate(l) if c == "S"
    )
    grid = [l[:] for l in inputs]
    loop, _ = visit_loop(animal_pos, grid)
    for i in range(len(grid)):
        for j in range(len(grid[0])):
            if (i, j) not in loop:
                grid[i][j] = " "
    visited = {animal_pos}
    info("\n" + hilight(grid, {animal_pos}))
    first = list(connecting_neighbors(animal_pos, grid))
    (l, dl), (r, dr) = first
    flood_left = set()
    flood_right = set()
    while l != r:
        visited.add(l)
        visited.add(r)
        # dbg(f"\n" + hilight(grid, visited))
        flood_fill(l, dl, grid, flood_left, flood_right)
        flood_fill(r, dr, grid, flood_right, flood_left)
        l, dl = next(p for p in connecting_neighbors(l, grid) if p[0] not in visited)
        r, dr = next(p for p in connecting_neighbors(r, grid) if p[0] not in visited)

    for p in flood_left:
        if grid[p[0]][p[1]] in PIPES:
            raise ValueError(f"left flood fill has covered a pipe at {p=}")
        grid[p[0]][p[1]] = "X"
    for p in flood_right:
        if grid[p[0]][p[1]] in PIPES:
            raise ValueError(f"right flood fill has covered a pipe at {p=}")
        if grid[p[0]][p[1]] == "X":
            raise ValueError(f"right flood fill has covered a left flood at {p=}")
        grid[p[0]][p[1]] = "."

    info("final:\n" + hilight(grid, set()))
    warn(f"left  (X): {len(flood_left)}")
    warn(f"right (.): {len(flood_right)}")
    edge = next(
        (i, j) for i, l in enumerate(grid) for j, c in enumerate(l) if c not in PIPES
    )
    if edge in flood_left:
        return len(flood_right)
    elif edge in flood_right:
        return len(flood_left)
    else:
        raise ValueError(f"edge {edge=} is not in flood fill")


if __name__ == "__main__":
    main()
