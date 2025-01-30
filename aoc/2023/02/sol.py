#!/usr/bin/env python3
import re


def input_lines():
    # read stdin
    while True:
        try:
            yield input()
        except EOFError:
            break

def sol_1():
    r = re.compile(r'(\d+) red')
    g = re.compile(r'(\d+) green')
    b = re.compile(r'(\d+) blue')
    tot = 0
    for i,line in enumerate(input_lines(), 1):
        if not line:
            continue
        reds = r.findall(line)
        greens = g.findall(line)
        blues = b.findall(line)
        good = all(int(r) <= 12 for r in reds) and all(int(g) <= 13 for g in greens) and all(int(b) <= 14 for b in blues)
        print(f'game {i}: {good=} {reds=} {greens=} {blues=}')
        if good :
            tot += i
    print(tot)

def sol_2():
    r = re.compile(r'(\d+) red')
    g = re.compile(r'(\d+) green')
    b = re.compile(r'(\d+) blue')
    tot = 0
    for i,line in enumerate(input_lines(), 1):
        if not line:
            continue
        reds = r.findall(line)
        greens = g.findall(line)
        blues = b.findall(line)
        minred = max(int(r) for r in reds)
        mingreen = max(int(g) for g in greens)
        minblue = max(int(b) for b in blues)
        print(f'game {i}: {reds=} {greens=} {blues=}')
        tot +=  minred * mingreen * minblue
    print(tot)

if __name__ == '__main__':
    sol_2()

