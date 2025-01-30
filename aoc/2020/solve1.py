
import sys
from itertools import combinations

def main():
    numbers = { int(n) for n in sys.stdin.read().split() }
    for m,n in combinations(numbers, 2):
        diff = 2020 - n - m
        if diff in (m,n):
            continue
        if diff in numbers:
            return n*m*diff
    else:
        sys.exit()

print(main())


