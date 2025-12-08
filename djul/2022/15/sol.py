words = []
with open('../12/SCOWL-wl/words.txt') as f:
    for l in f:
        w = l.strip()
        if w.isalpha() and len(w) > 4:
            words.append(w.lower())

with open('problem.dat') as f:
    problem = f.read().lower()


def fits(plain: str, broken: str) -> bool:
    for (a, b) in zip(plain, broken):
        if b.isalpha() and b != a:
            return False
    return True


def matches(word: str, text: str) -> bool:
    l = len(word)
    for i in range(len(text) - l):
        if fits(word, text[i:i + l]):
            return True
    return False


print(f"{len(words)} words to process")
solutions = [w for w in words if matches(w, problem)]
sols = sorted(solutions, key=lambda x: len(x), reverse=True)
print(sols[:100])
