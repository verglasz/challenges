#!/usr/bin/env python3

import re
import sys
from collections import defaultdict

eprint = lambda *args, **kwargs: print(*args, file=sys.stderr, **kwargs)

def intput_lines():
    while True:
        try:
            yield  input()
        except EOFError:
            break

def bbox(i, s,e, maxx, maxy):
    x0 = s - 1 if s > 0 else 0
    y0 = i - 1 if i > 0 else 0
    x1 = e + 1 if e < maxx else maxx
    y1 = i + 1 if i < maxy else maxy
    return (x0, y0, x1, y1)

def explore_bbox(lines, bbox):
    x0, y0, x1, y1 = bbox
    for j in range(y0, y1+1):
        yield from lines[j][x0:x1+1]

def explore_bbox_c(lines, bbox):
    x0, y0, x1, y1 = bbox
    for j in range(y0, y1+1):
        for i in range(x0, x1+1):
            yield lines[j][i], i,j

def explore_lines(lines, bbox):
    x0, y0, x1, y1 = bbox
    for j in range(y0, y1+1):
        yield lines[j][x0:x1+1]

class bcolors:
    HEADER = '\033[95m'
    OKBLUE = '\033[94m'
    OKCYAN = '\033[96m'
    OKGREEN = '\033[92m'
    WARNING = '\033[93m'
    FAIL = '\033[91m'
    ENDC = '\033[0m'
    BOLD = '\033[1m'
    UNDERLINE = '\033[4m'

def sol2():
    # nums = re.compile(r'(\d+)')
    # gears = re.compile(r'\*')
    # inputs = list(intput_lines())
    # total = 0
    # for i,line in enumerate(inputs):
    #     for g in gears.finditer(line):
    #         s = g.start()
    #         bb = bbox(i, s, s, len(line)-1, len(inputs)-1)
    #         parts = [nums.findall(l) for l in explore_lines(inputs, bb)]
    #         eprint(line, s, bb, parts)
    #         if sum(len(p) for p in parts) = 1:
    #             total += 1
    #         if all(x != '.' and not x.isdigit() for x in explore_bbox(inputs, bb)):
    #             total += 1


    #     s, e = n.start(), n.end()
    #     bb = bbox(i, s,e-1, len(line)-1, len(inputs)-1)
    #     eprint(line, bb)
    #     eprint(list(explore_bbox(inputs, bb)), n)
    #     if all(x != '.' and not x.isdigit() for x in explore_bbox(inputs, bb)):
    #         total += int(n.group(0))
    nums = re.compile(r'(\d+)')
    inputs = [l for l in intput_lines() if l]
    total = 0
    adj = defaultdict(list)
    for linei,line in enumerate(inputs):
        for n in nums.finditer(line):
            s, e = n.start(), n.end()
            bb = bbox(linei, s, e-1, len(line)-1, len(inputs)-1)
            bbc = list(explore_bbox_c(inputs, bb))
            eprint(f'{n.group():>5} in {line}')
            fsti, fstj = bbc[0][1:]
            box = [list(l) for l in explore_lines(inputs, bb)]
            for c, i,j in bbc:
                if c == '*':
                    x, y = i - fsti, j - fstj
                    box[y][x] = f'{bcolors.BOLD}{bcolors.FAIL}{box[y][x]}{bcolors.ENDC}'
                    eprint('\n'.join(''.join(l) for l in box))
                    eprint(f'adding {n.group()} to {i,j}')
                    adj[i,j].append(int(n.group()))
    eprint(adj)
    for k,v in adj.items():
        if len(v) == 2:
            total += v[0]*v[1]

    print(total)

def sol1():
    nums = re.compile(r'(\d+)')
    inputs = list(intput_lines())
    total = 0
    for i,line in enumerate(inputs):
        for n in nums.finditer(line):
            s, e = n.start(), n.end()
            bb = bbox(i, s,e-1, len(line)-1, len(inputs)-1)
            eprint(line, bb)
            eprint(list(explore_bbox(inputs, bb)), n)
            if any(x != '.' and not x.isdigit() for x in explore_bbox(inputs, bb)):
                total += int(n.group(0))

    print(total)

if __name__ == '__main__':
    sol2()

