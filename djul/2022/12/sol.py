forbidden = set('auistny')


def hangman_check(word):
    if len(word) != 4:
        return False
    a, b, c, d = word
    return a == 'z' and d == 'o' and b not in forbidden and c not in forbidden


words = []
with open('SCOWL-wl/words.txt') as f:
    allwords = [l.strip() for l in f]

for line in allwords:
    if hangman_check(line.strip()):
        words.append(line.strip())


def rotate(c: int, n: int) -> int:
    new = c - 0x61 + n
    return new % 26 + 0x61  # ASCII a is 0x61 (dec 97)


other = bytes(rotate(c, 6) for c in b'acceleration').decode()[9:]

worddict = set(allwords)

prefixd = [w[:2] + other for w in ('true', 'one', 'yes')]

print(next(w for w in prefixd if w in worddict))
