#!/usr/bin/env python3

# Or you could first layout the nodes with the edges which determine the correct order of the nodes only:
#
# digraph g1 {
#     node [shape = doublecircle]; N4 N6;
#     node [shape = circle];
#     edge[label="{1,0}"];
#     N0 -> N1 -> N2 -> N3 -> N4 -> N5 -> N6 -> N0;
# }
#
# with:
#
# circo graph.gv > tempgraph.gv
#
# Then add the remaining edges to tempgraph.gv - just copy-paste the following before the closing }:
#
# N0 -> N4 [ label = "{1,0}"];
# N1 -> N5 [ label = "{1,0}"];
# N2 -> N6 [ label = "{1,0}"];
# N3 -> N0 [ label = "{1,0}"];
# N4 -> N1 [ label = "{1,0}"];
# N5 -> N2 [ label = "{1,0}"];
# N6 -> N3 [ label = "{1,0}"];
#
# And render it with neato and the -n option:
#
# neato -n tempgraph.gv -Tpng -O
#
# You may want to fine-tune the position of the labels:
#
# key = {
#     8:18,
#     18:7,
#     7:9,
#     9:17,
#     17:25,
#     25:19,
#     19:15,
#     15:0,
#     0: 14,
# }
#

alphabet = "abcdefghijklmnopqrstuvwxyz"


def ansi(s):
    return f"\033[1;31m{s}\033[0m"


def highlight(s: str, k: dict) -> tuple[str, str, str, str, str]:
    up = [" " for _ in s]
    down = [" " for _ in s]
    a = [c for c in s]
    b = [c for c in s]
    mid = [c for c in s]
    for i, _ in enumerate(s):
        if i in k:
            up[i] = s[k[i]]
            down[k[i]] = s[i]
            mid[i] = ansi(mid[i])
            mid[k[i]] = ansi(mid[k[i]])
            a[i] = ansi(a[i])
            b[k[i]] = ansi(b[k[i]])

    return "".join(up), "".join(a), "".join(mid), "".join(b), "".join(down)


def otherloops():
    seq = []
    c = 8
    while c is not None:
        seq.append(c)
        c = key.get(c, None)
    alpha = alphabet.upper() * 2
    for i in range(26):
        print(f"{i:2}:", "".join(alpha[c + i] for c in seq))
        # print(''.join(alpha[c] for c in seq[::-1]))
        print()


def idk():
    for i in range(26):
        alpha = alphabet[i:] + alphabet[:i]
        things = highlight(alpha, key)
        print(*things, sep="\n", end="\n\n")


base_graph = """
	 0 -> 1;
	 1 -> 2;
	 2 -> 3;
	 3 -> 4;
	 4 -> 5;
	 5 -> 6;
	 6 -> 7;
	 7 -> 8;
	 8 -> 9;
	 9 -> 10;
	10 -> 11;
	11 -> 12;
	12 -> 13;
	13 -> 14;
	14 -> 15;
	15 -> 16;
	16 -> 17;
	17 -> 18;
	18 -> 19;
	19 -> 20;
	20 -> 21;
	21 -> 22;
	22 -> 23;
	23 -> 24;
	24 -> 25;
	25 -> 0;

	0 -> 14;
	15 -> 0;
	19 -> 15;
	18 -> 7;
	25 -> 19;
	17 -> 25;
	9 -> 17;
	7 -> 9;
	8 -> 18;
"""


def multidot():
    with open("./positioned.dot", "r") as f:
        base = f.read()
    for i in range(26):
        alpha = alphabet[i:] + alphabet[:i]
        labels = "\n".join(
            f'\t{j:2} [label="{c}"];' for j, c in enumerate(alpha.upper())
        )
        new = base.replace('node [label="\\N"];', labels)
        with open(f"graphs/{i:02}.dot", "w") as f:
            f.write(new)
        # with open(f'graphs/{i:02}.dot', 'w') as f:
        #     f.write('digraph {\n')
        #     for j, c in enumerate(alpha):
        #         f.write(f'\t{j:2} [label="{c.upper()}"];\n')
        #     f.write(base_graph)
        #     f.write('}\n')


def main():
    otherloops()


if __name__ == "__main__":
    main()
