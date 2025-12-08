from dataclasses import dataclass
import re
import sys

class Constraint:
    def __or__(self, other: 'Constraint'):
        match (self,other):
            case (Literal(True), _) | (_,Literal(True)):
                return myTrue
            case (Literal(False), x) | (x, Literal(False)):
                return x
            case (Or(a), Or(b)):
                return Or(a+b)
            case (Or(ors), other) | (other, Or(ors)):
                return Or(ors + [other])
            case (a,b): # only Leafs and Ands
                return Or([a,b])

    def __and__(self, other: 'Constraint'):
        match (self,other):
            case (Literal(True), x) | (x, Literal(True)):
                return x
            case (Literal(False), _) | (_, Literal(False)):
                return myFalse
            case (And(a), And(b)):
                return And(a+b)
            case (Or(ors), other) | (other, Or(ors)):
                return Or([x & other for x in ors])
            case (And(ands), Leaf(_) as x) | (Leaf(_) as x, And(ands)):
                return And(ands + [x])
            case (Leaf(_,_) as a, Leaf(_,_) as b): # only Leafs
                return And([a,b])

@dataclass
class Literal(Constraint):
    val: bool

    def render(self, length=None):
        return str(self)

myTrue = Literal(True)
myFalse = Literal(False)

@dataclass
class Leaf(Constraint):
    idx: int
    val: str | None

    def render(self, _length=None):
        return f'{self.val!r} at {self.idx}'

@dataclass
class And(Constraint):
    of: list[Leaf]

    def render(self, length=None):
        if length is None:
            lengths = [val.idx for val in self.of if val.val is None]
            if len(lengths) == 1:
                length = lengths[0]
            else:
                length = max(val.idx for val in self.of) + 1
        candidate: list[str] = ['\u2588'] * length
        for v in self.of:
            if v.val is not None:
                candidate[v.idx] = v.val
        return ''.join(candidate)

@dataclass
class Or(Constraint):
    of: list[And | Leaf]

    def render(self, length=None):
        return '\n'.join(x.render(length) for x in self.of)

@dataclass
class CheatCompare:
    idx: int

    def __eq__(self, val: str):
        return Leaf(self.idx, val)

class Impostor:
    def __getitem__(self, idx):
        return CheatCompare(idx)

def parse(filename)-> list[str]:
    statements = []
    with open(filename) as f:
        for line in f:
            cleaned = re.sub(
               r'''(s\[[^'"]+["'][^'"]*['"])''',
               r"(\1)",
                line.strip()
                   .replace("const ", "")
                   .replace("verifyPassword =", "verifyPassword = (")
                   .replace("s =>", "lambda s: (")
                   .replace(";", ");")
                   .replace("||", ") | (")
                   .replace("&&", "&")
                   .replace("false", "myFalse")
                   .replace("true","myTrue")
                   .replace("undefined", "None")
            )
            statements.append(cleaned)
    return statements

def get_verifier(statements: list[str]):
    statements.insert(0, "def generated_function():")
    statements.append('return verifyPassword;')
    source = '\n    '.join(statements)
    code = compile(source, '<parsed problem.js>', 'exec')
    myglobals= {'myTrue': myTrue, 'myFalse':myFalse}
    exec(code, myglobals)
    return myglobals['generated_function']()

problem = sys.argv[1] if len(sys.argv) > 1 else 'problem.js'
verifyPassword = get_verifier(parse(problem))
result = verifyPassword(Impostor())
print(result.render())

