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


def ways(time, distance):
    beats = 0
    while time > beats and (time - beats) * beats <= distance:
        beats += 1
    end = beats
    while time > end and (time - end) * end > distance:
        end += 1
    interval = end - beats
    eprint(f"{time=} {distance=} {beats=} {end=}, {interval=}")
    return interval


def fast_ways(time, distance):
    delta = time * time - 4 * distance
    if delta < 0:
        return 0
    delta = delta**0.5
    beats = (time - delta) / 2
    end = (time + delta) / 2
    floor_end = int(end)
    ceil_beats = int(beats)
    if ceil_beats < beats:
        ceil_beats += 1
    end = floor_end + 1
    beats = ceil_beats if ceil_beats > 0 else 0
    interval = end - beats
    eprint(f"{time=} {distance=} {beats=} {end=}, {interval=}")
    return interval


def sol1():
    times = input()
    assert times.startswith("Time:")
    time = [int(x.strip()) for x in times[len("Time:") :].split()]
    distance = input()
    assert distance.startswith("Distance:")
    distance = [int(x.strip()) for x in distance[len("Distance:") :].split()]
    assert len(time) == len(distance)
    res = 1
    for t, d in zip(time, distance):
        res *= ways(t, d)
    return res


def fast_sol2(inp):
    times = inp[0]
    assert times.startswith("Time:")
    time = int(times[len("Time:") :].replace(" ", ""))
    distance = inp[1]
    assert distance.startswith("Distance:")
    distance = int(distance[len("Distance:") :].replace(" ", ""))
    res = fast_ways(time, distance)
    return res


def sol2(inp):
    times = inp[0]
    assert times.startswith("Time:")
    time = int(times[len("Time:") :].replace(" ", ""))
    distance = inp[1]
    assert distance.startswith("Distance:")
    distance = int(distance[len("Distance:") :].replace(" ", ""))
    res = ways(time, distance)
    return res


def main():
    inp = list(input_lines())
    # time the solutions
    import timeit

    fs = fast_sol2(inp)
    eprint(f"Fast sol {fs}")
    s = sol2(inp)
    eprint(f"Sol {s}")
    if fs != s:
        eprint("Solutions differ!")
    else:
        eprint("Solutions match! running timeit")
        gb = globals()
        gb["inp"] = inp
        tf = timeit.timeit("fast_sol2(inp)", globals=gb, number=1000)
        eprint(f"Fast sol time: {tf}")
        ts = timeit.timeit("sol2(inp)", globals=gb, number=3)
        eprint(f"Sol time: {ts}")


if __name__ == "__main__":
    fast_sol2(list(input_lines()))
