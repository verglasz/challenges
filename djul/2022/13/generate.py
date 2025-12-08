from collections.abc import Iterable
import sys
import random
from types import EllipsisType
import string


def rand_char():
    # 0x61 = ord('a'), 0x7a = ord('z')
    return random.randint(0x61, 0x7a)


def random_name(length: int) -> str:
    bs = bytes(rand_char() for _ in range(length))
    return bs.decode()


def fresh_zero_func(name: str) -> str:
    return f'const {name} = s => false;'


CHECKS = [repr(c) for c in string.printable] + ['undefined']

def str_check(idx: int, val: str | EllipsisType | None = ...):
    if val is ...:
        idx = random.randint(0, idx)
        val = random.choice(CHECKS)
    elif val is None:
        val = 'undefined'
    else:
        val = repr(val)
    return f"s[{idx}] == {val}"


def or_stmt(parts: Iterable[str])-> str:
    return ' || '.join(parts)

def and_stmt(parts: Iterable[str])-> str:
    return ' && '.join(parts)

def fcall(name: str) ->str:
    return f'{name}(s)'

def new_and(must_zero: int, zeros: list[str], noise: list[str], max_idx: int, cap: int = 8) -> tuple[int, int,str]:
    parts = []
    for _ in range(random.randrange(1,cap)):
        match random.randrange(3):
            case 0:
                must_zero = -1
                parts.append(fcall(random.choice(zeros)))
            case 1:
                parts.append(fcall(random.choice(noise)))
            case 2:
                parts.append(str_check(max_idx))
    if must_zero > 0:
        repl = random.randrange(len(parts))
        parts[repl] = fcall(random.choice(zeros))
        must_zero = -1
    return must_zero, len(parts), and_stmt(parts)

def new_or(must_zero: int, zeros: list[str], noise: list[str], max_idx: int, cap: int = 20) -> tuple[int,str]:
    tot = 0
    parts = []
    is_zero = True
    while tot < cap:
        z, n, a = new_and(must_zero, zeros, noise, max_idx)
        is_zero &= z < 0
        tot += n
        parts.append(a)
    return is_zero, or_stmt(parts)

def new_func(must_zero: int, zeros: list[str], noise: list[str], max_idx: int):
    z, body = new_or(must_zero, zeros, noise, max_idx)



if __name__ == '__main__':
    size = int(sys.argv[1])

