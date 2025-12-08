from collections.abc import Callable, Iterable, Sequence
import itertools as it
from typing import TypeVar

seq = '1210101210121012101012'


def mirror3(seq: Iterable[int]):
    return [(2 - i) for i in seq]


def shift(n: int) -> Callable[[Iterable[int]], Iterable[int]]:
    return lambda seq: [(i + n) % 3 for i in seq]


def trans(s: Iterable[int]):
    as_str = ''.join(str(i) for i in s)
    n = int(as_str, 3)
    return (f'{n:x}', n.to_bytes(5, 'little'))


T = TypeVar("T")
R = TypeVar("R")


def multitransform(transforms: list[Callable[[T], T]], seq: Iterable[T]):
    build = lambda t: lambda x: [t(x), x]
    for t in transforms:
        seq = it.chain.from_iterable(map(build(t), seq))
    return seq


things = list(
    map(
        trans,
        multitransform(
            [mirror3, shift(1), shift(2), lambda i: i[::-1]],
            [[int(c) for c in seq]])))
