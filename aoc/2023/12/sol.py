#!/usr/bin/env python3

import argparse
from collections import namedtuple
import logging
import sys
import time
from typing import Any, Generator, NamedTuple

eprint = lambda *args, **kwargs: print(*args, file=sys.stderr, **kwargs)

logging.basicConfig(level=logging.DEBUG)
logger = logging.getLogger(__name__)

dbg, info, warn, err = logger.debug, logger.info, logger.warning, logger.error


class bcolors:
    HEADER = "\033[95m"
    OKBLUE = "\033[94m"
    OKCYAN = "\033[96m"
    OKGREEN = "\033[92m"
    WARNING = "\033[93m"
    FAIL = "\033[91m"
    ENDC = "\033[0m"
    BOLD = "\033[1m"
    UNDERLINE = "\033[4m"


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
        start = time.time()
        print(sol2(inputs))
        end = time.time()
        eprint(f"sol 2 took {(end-start)*1000:.2f}ms")


def parse():
    lines = []
    for l in input_lines():
        if not l:
            continue
        springs, nums = l.split(" ")
        nums = [int(n) for n in nums.split(",")]
        lines.append((springs, nums))
    return lines


def block_tails(block: str, n: int, depth=0) -> list[str]:
    # prefix = "  " * depth
    # dbgp = lambda fs, *args, **kwargs: dbg(f"{prefix}{fs}", *args, **kwargs)
    # dbgp(f"trying to fit {n} in block {block}")
    if n > len(block):
        # dbgp(f"not enough space")
        return []
    tails = []
    start = 0
    for start in range(len(block) - n):
        end = start + n
        # dbgp(
        #     f"trying start {start} {block[:start]}{bcolors.BOLD}{bcolors.FAIL}{block[start:end]}{bcolors.ENDC}{block[end:]}"
        # )
        if block[end] != "#":
            # suitable position
            # dbgp(
            #     f"remains {block[:end+1]}{bcolors.BOLD}{bcolors.OKGREEN}{block[end+1:]}{bcolors.ENDC}"
            # )
            tail = block[end + 1 :]
            tails.append((tail))
        else:
            # we need to go further forward
            # not continue because of the coming check
            pass
        if block[start] == "#":
            # we can't go past this one
            break
    else:
        # we reached the end, include the end
        tails.append((""))
    # dbgp(f"allocated {n} with these possible tails: {tails}")
    return tails


def count1(blocks: list[str], nums: list[int], depth=0) -> int:
    prefix = "  " * depth
    dbgp = lambda fs, *args, **kwargs: dbg(f"{prefix}{fs}", *args, **kwargs)
    if not nums:
        # dbgp(f"no more broken to be allocated among {blocks}")
        ways = not any("#" in block for block in blocks)
        # dbgp(f"returning {int(ways)}")
        return int(ways)

    ways = 0
    for k, block in enumerate(blocks):
        n = nums[0]
        tails = block_tails(block, n, depth)
        for t in tails:
            if t:
                next_blocks = [t] + blocks[k + 1 :]
            else:
                next_blocks = blocks[k + 1 :]
            ways += count1(next_blocks, nums[1:], depth + 1)
        if "#" in block:
            # we can't go past this one
            break

    return ways


def count1memo(blocks: list[str], nums: list[int], memo: dict, depth=0) -> int:
    # prefix = "  " * depth
    # dbgp = lambda fs, *args, **kwargs: dbg(f"{prefix}{fs}", *args, **kwargs)
    args = (tuple(blocks), tuple(nums))
    if args in memo:
        return memo[args]

    if not nums:
        # dbgp(f"no more broken to be allocated among {blocks}")
        ways = not any("#" in block for block in blocks)
        # dbgp(f"returning {int(ways)}")
        return int(ways)

    ways = 0
    for k, block in enumerate(blocks):
        n = nums[0]
        tails = block_tails(block, n, depth)
        for t in tails:
            if t:
                next_blocks = [t] + blocks[k + 1 :]
            else:
                next_blocks = blocks[k + 1 :]
            ways += count1memo(next_blocks, nums[1:], memo, depth + 1)
        if "#" in block:
            # we can't go past this one
            break

    memo[args] = ways
    return ways


def count2(block: str, blocks: list[str], nums: list[int], depth=0) -> int:
    # prefix = "  " * depth
    # dbgp = lambda fs, *args, **kwargs: dbg(f"{prefix}{fs}", *args, **kwargs)
    if not nums:
        ways = not any("#" in block for block in blocks)
        # dbgp(f"no more broken to be allocated among {blocks}, giving {int(ways)}")
        return int(ways)
    if "#" in block:
        # dbgp(f"block {block} must be consumed")
        n, *num_tail = nums
        tails = block_tails(block, n, depth)
        total = 0
        for t in tails:
            if t:
                total += count2(t, blocks, num_tail, depth + 1)
            elif not blocks:
                total += int(len(num_tail) == 0)
            else:
                total += count2(blocks[0], blocks[1:], num_tail, depth + 1)

    elif not blocks:
        total = full_free_combinations(len(block), nums)
        # dbgp(f"block {block} is the last, {total} way of putting {nums}")
    else:
        # dbgp(f"block {block} is freely allocatable")
        total = 0
        next_block, *next_blocks = blocks
        for ways, num_tail in free_combinations(len(block), nums, depth + 1):
            # dbg(f"trying {num_tail} on {blocks}")
            ways *= count2(next_block, next_blocks, num_tail, depth + 2)
            total += ways
    # dbgp(f"gave {total}")
    return total


def fact(n: int, stop=1) -> int:
    res = 1
    while n > stop:
        res *= n
        n -= 1
    return res


def pick(n: int, k: int) -> int:
    m = n - k
    if m < k:
        k, m = m, k
    if n < m:
        return 0
    over = fact(n, m)
    under = fact(k)
    # dbg(f"pick({n}, {k}): {over=} {under=}")
    return over // under


def full_free_combinations(length: int, nums: list[int]) -> int:
    l = len(nums)
    s = sum(nums)
    spare = length - s - l + 1
    # dbg(f"full_free_combinations({length}, {nums}): {spare=} {l=} {s=}")
    if spare < 0:
        return 0
    return pick(spare + l, l)


def free_combinations(
    length: int, nums: list[int], depth=0
) -> Generator[tuple[int, list[int]], Any, Any]:
    # prefix = "  " * depth
    # dbgp = lambda fs, *args, **kwargs: dbg(f"{prefix}{fs}", *args, **kwargs)
    # dbgp(f"free_combinations({length}, {nums}) giving: (1, {nums})")
    yield 1, nums
    for i, n in enumerate(nums, 1):
        ways = full_free_combinations(length, nums[:i])
        # dbgp(f"{ways} if we allocate {nums[:i]} ({nums[i:]} left)")
        if ways == 0:
            break
        yield ways, nums[i:]


class Split(NamedTuple):
    pre: tuple[str, int]
    post: tuple[str, int]


def splits(block: str, nums: list[int]) -> list[Split]:
    ...


def count_memo(
    block: str, blocks: list[str], nums: list[int], memo: dict, depth=0
) -> int:
    args = (block, tuple(blocks), tuple(nums))
    if args in memo:
        return memo[args]
    if not nums:
        ways = not any("#" in block for block in blocks)
        # dbgp(f"no more broken to be allocated among {blocks}, giving {int(ways)}")
        return int(ways)
    if 1 or "#" in block:
        # dbgp(f"block {block} must be consumed")
        n, *num_tail = nums
        tails = block_tails(block, n, depth)
        total = 0
        if "#" not in block and blocks:
            total += count_memo(blocks[0], blocks[1:], nums, memo, depth + 1)
        for t in tails:
            if t:
                total += count_memo(t, blocks, num_tail, memo, depth + 1)
            elif not blocks:
                total += int(len(num_tail) == 0)
            else:
                total += count_memo(blocks[0], blocks[1:], num_tail, memo, depth + 1)

    elif not blocks:
        total = full_free_combinations(len(block), nums)
        # dbgp(f"block {block} is the last, {total} way of putting {nums}")
    else:
        # dbgp(f"block {block} is freely allocatable")
        total = 0
        next_block, *next_blocks = blocks
        for ways, num_tail in free_combinations(len(block), nums, depth + 1):
            # dbg(f"trying {num_tail} on {blocks}")
            ways *= count_memo(next_block, next_blocks, num_tail, memo, depth + 2)
            total += ways
    # dbgp(f"gave {total}")
    memo[args] = total
    return total

    ...


def count3(blocks: list[str], nums: list[int]) -> int:
    # fst_broken = block.find("#")
    # if fst_broken == -1:
    #     tot = 0
    #     for i in range(len(nums) + 1):
    #         ways_fst = full_free_combinations(len(block), nums[:i])
    #         if ways_fst == 0:
    #             break
    #         ways_snd = count_memo(blocks[0], blocks[1:], nums[i:], memo)
    #         tot += ways_fst * ways_snd
    ...


def sol1(inputs):
    warn(f"{len(inputs)} inputs")
    info(inputs)

    tot = 0

    for i, (springs, nums) in enumerate(inputs):
        if i % 10 == 0:
            warn(f"at {i}/{len(inputs)}")
        all_blocks = [k for k in springs.split(".") if k]
        block, *blocks = all_blocks
        # info(f"[{' '.join(all_blocks)}] {nums} counting")
        # loc = count_memo(block, blocks, nums, {})
        loc = count1memo(all_blocks, nums, {})
        # info(f"[{' '.join(all_blocks)}] {nums} -> {loc}")
        tot += loc
    return tot


def unfold(blocks, nums, times):
    return "?".join(blocks for _ in range(times)), nums * times


def sol2(inputs):
    inputs = [unfold(blocks, nums, 5) for blocks, nums in inputs]
    return sol1(inputs)


if __name__ == "__main__":
    main()
