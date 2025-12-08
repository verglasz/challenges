#!/usr/bin/env python3

import itertools as it


class FenwickTree:
    def __init__(self, n):
        self.n = n
        self.tree = [0] * (n + 1)

    def update(self, x, v):
        while x <= self.n:
            self.tree[x] += v
            x += x & -x

    def query(self, x):
        ans = 0
        while x:
            ans += self.tree[x]
            x -= x & -x
        return ans


TREE = [0, 67, 164, 55, 316, 108, 156, 103, 692, 51]

fw = FenwickTree(9)
fw.tree = TREE[:]

xx = [fw.query(i) for i in range(0, 10)]
sol = bytes(b - a for a, b in it.pairwise(xx))

print(sol)
