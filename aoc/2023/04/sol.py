#!/usr/bin/env python3

import sys


eprint = lambda *args, **kwargs: print(*args, file=sys.stderr, **kwargs)

def intput_lines():
    while True:
        try:
            yield  input()
        except EOFError:
            break

def sol1():
    tot = 0
    for line in intput_lines():
        if not line:
            continue
        _, line = line.split(':')
        win, have  = line.split("|")
        winset = set(s.strip() for s in win.split())
        points = 0
        for word in have.split():
            if word.strip() in winset:
                points += 1
        if points > 0:
            tot += 2**(points-1)
    print(tot)


def sol2():
    inputs = [l for l in intput_lines() if l]
    total = len(inputs)
    copies = {}
    for i,line in enumerate(inputs):
        _, line = line.split(':')
        win, have  = line.split("|")
        winset = set(s.strip() for s in win.split())
        count = sum(1 for word in have.split() if word.strip() in winset)
        eprint(f'{i}: {count}')
        dels = []
        partial = 0
        for c,v in copies.items():
            if i <= c:
                partial += v
            if i >= c:
                dels.append(c)
        for d in dels:
            del copies[d]
        eprint(f'using {partial} copies of this one')
        eprint(f'incrementing until {i+count}')
        total += partial
        if count > 0:
            copies[i+count] = copies.get(i+count,0) + partial+1
    print(total)

sol2()

