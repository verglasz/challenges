#!/usr/bin/env python3

import sys
import re


def input_lines():
    """Read lines from input."""
    return [line.strip() for line in sys.stdin if line]

def hilight(line: str, tolight: set[str]) -> str:
    """Highlight lines."""
    pattern = '|'.join(tolight)
    line = re.sub(rf'({pattern})', r'\033[1;31m\1\033[0m', line)
    return line


def dotify(lines: list[str]):
    print('digraph G {')
    for line in lines:
        x = re.match(r'(%|&)?(.*) -> (.*)', line)
        if x is None:
            continue
        typ = x.group(1)
        src = x.group(2)
        dests = x.group(3).split(', ')
        if typ == '&':
            print(f'  {src} [shape=box];')
        elif typ == '%':
            print(f'  {src} [shape=circle];')
        else:
            print(f'  {src} [shape=triangle];')

        print(f'  {src} -> {{ {" ".join(dests)} }};')

    print('}')

def find_hilight(lines: list[str], mark: str) -> set[str]:
    tolight = set()
    for line in lines:
        x = re.match(rf'^{mark}(..)', line)
        if x:
            tolight.add(x.group(1))
    return tolight

def main():
    """Main program."""
    lines = input_lines()
    if len(sys.argv) > 1:
        mark= sys.argv[1]
        tolight = find_hilight(lines, mark)
        for line in lines:
            print(hilight(line, tolight))
    else:
        dotify(lines)

if __name__ == '__main__':
    main()

