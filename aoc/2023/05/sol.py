#!/usr/bin/env python3

from collections import namedtuple
import sys
from types import SimpleNamespace


eprint = lambda *args, **kwargs: print(*args, file=sys.stderr, **kwargs)


def input_lines():
    while True:
        try:
            yield input()
        except EOFError:
            break


def parse():
    line = input()
    assert line.startswith("seeds: ")
    line = line[len("seeds: ") :]
    seeds = [int(s.strip()) for s in line.split()]
    assert input() == ""
    mappings = [
        parse_map("seed-to-soil"),
        parse_map("soil-to-fertilizer"),
        parse_map("fertilizer-to-water"),
        parse_map("water-to-light"),
        parse_map("light-to-temperature"),
        parse_map("temperature-to-humidity"),
        parse_map("humidity-to-location"),
    ]
    for m in mappings:
        m.sort(key=lambda x: x.src)
    return seeds, mappings


Map = namedtuple("Map", ["src", "dest", "length"])


def parse_map(name: str):
    mapping = []
    assert input() == f"{name} map:"
    line = input()
    eprint(name, line)
    while line:
        dest, src, length = tuple(int(s.strip()) for s in line.split())
        mapping.append(Map(src, dest, length))
        line = input()
        eprint(name, line)
    return mapping


def resolve(mapping: list[Map], src: int) -> int:
    for m in mapping:
        delta = src - m.src
        if 0 <= delta < m.length:
            return m.dest + delta
    return src


def full_resolve(mappings: list[list[Map]], src: int) -> int:
    seed = src
    eprint(f'mapping seed {seed}')
    for mapping in mappings:
        src = resolve(mapping, src)
        eprint(f'-> {src:3}', end=' ')
    eprint(f'seed {seed:3} mapped to {src:3}')
    return src



def sol1():
    # inputs = list(input_lines())
    # assert inputs[-1] == ''
    seeds, mappings = parse()
    eprint(seeds)
    locs = {s: full_resolve(mappings, s) for s in seeds}
    eprint(locs)
    # minloc = min(full_resolve(mappings, s) for s in seeds)
    print(min(locs.values()))

def range_resolve(mapping: list[Map], start, end) -> list[tuple[int,int]]:
    dest = []
    for m in mapping:
        if start >= end:
            break
        if start < m.src:
            mapd_end = min(end, m.src)
            dest.append((start, mapd_end))
            start = mapd_end
            continue
        delta = start - m.src
        if delta >= m.length:
            # no overlap
            continue
        mapd_start = m.dest + delta
        mapd_range = min(m.length - delta, end - start)
        mapd_end = mapd_start + mapd_range
        dest.append((mapd_start, mapd_end))
        start = start + mapd_range
    if start < end:
        dest.append((start, end))
    return dest

def full_range_resolve(mappings: list[list[Map]], ranges: list[tuple[int,int]]) -> list[tuple[int,int]]:
    for mapping in mappings:
        new_ranges = []
        for start, end in ranges:
            new_ranges.extend(range_resolve(mapping, start, end))
        eprint('turned', ranges)
        eprint('into: ', new_ranges)
        ranges = new_ranges
    return ranges


def sol2():
    seeds, mappings = parse()
    assert len(seeds) % 2 == 0
    seed_ranges = []
    for i in range(0, len(seeds), 2):
        start, length = seeds[i], seeds[i+1]
        seed_ranges.append((start, start+length))
    loc_ranges = full_range_resolve(mappings, seed_ranges)
    print(min(start for start, end in loc_ranges))


sol2()

