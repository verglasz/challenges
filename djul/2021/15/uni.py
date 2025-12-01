
def weird(ch):
    return len(bytes(ch, 'utf8')) - 1

def getweird(s):
    for ch in s:
        if weird(ch):
            yield from bytes(ch, 'utf8')

def movcubits(charlist):
    b = bytearray(''.join(charlist), 'utf16')
    for i in range(0, len(b), 2):
        b[i] += 0x01
        b[i+1] += 0x20
    return b

def main(s):
    print(movcubits(getweird(s)).decode('utf16'))

