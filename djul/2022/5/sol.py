from collections.abc import Iterable, Iterator
import itertools
from typing import TypeVar
import sys

with open('text') as f:
    content = f.read()

since = 0
gaps = []
for c in content:
    if c == '0':
        since += 1
    else:
        gaps.append(since)
        since = 0

T = TypeVar('T')


def regroup(it: Iterable[T], cnt: int) -> Iterator[tuple[T]]:
    grouping = itertools.groupby(enumerate(it), lambda x: x[0] // cnt)
    groups = (tuple(l for _, l in g) for _, g in grouping)
    return groups


def strblock(s: str, cnt: int) -> Iterator[str]:
    return (''.join(s) for s in regroup(s, cnt))


def factor(x: int) -> list[int]:
    factors = []
    n = 2
    while n * n <= x:
        if (x % n) != 0:
            n += 1
            continue
        factors.append(n)
        x = x // n
        n = 2
    factors.append(x)
    return factors


p = 401
q = 101
n = p * q
t = (p - 1) * (q - 1)
e = 65537 % t
k = pow(e, -1, t)


def enc(m):
    return pow(m, e, n)


def dec(c):
    return pow(c, k, n)


tgroups = list(strblock(content, 8))
g401 = list(strblock(content, 401))
g101 = list(strblock(content, 101))
thebytes = [int(g, base=2) for g in tgroups]

i101 = [int(i, 2) for i in g101]
i401 = [int(i, 2) for i in g401]

sys.set_int_max_str_digits(40502)
huge = int(content, 2)

print('kek')
