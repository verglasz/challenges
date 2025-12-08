from collections.abc import Callable
import logging
from typing import Union
import pwn
import pwnlib
from pwn.toplevel import remote

import pwn


class PosRemote(remote):
    pos: int


ip = "34.240.255.211"
port = 4711


def to_pos(n):
    return divmod(n, 5)


def moves(frm, to):
    fr, fc = to_pos(frm)
    tr, tc = to_pos(to)
    while fr < tr:
        yield b'go down\n'
        fr += 1
    while fc < tc:
        yield b'go right\n'
        fc += 1
    while fr > tr:
        yield b'go up\n'
        fr -= 1
    while fc > tc:
        yield b'go left\n'
        fc -= 1


def getindex(p: PosRemote, n: int):
    try:
        p.send(b'getfat %d\n' % n)
    except:
        pass
    p.recvuntil(b'within the fat you find a... ')
    idx = int(p.recvline(keepends=False), 16)
    return idx


def bincontent(p: PosRemote):
    # p.recvuntil(
    #     b"It is a file cluser,\nyou can view its contents with: showcontent\n")
    p.recvuntil(b'There are exits in the folowing directions:\n')
    skipsection(p)
    try:
        p.send(b'showcontent\n')
    except:
        pass
    pwn.log.debug('after showcontent skipping: {}'.format(p.recvline()))
    bs = p.recvline(keepends=False)
    return bytes(int(x, 16) for x in bs.split())


def lsfiles(p: PosRemote) -> list[dict]:
    content = []
    p.recvuntil(
        b"It is a folder cluster and contains:\nNAME     EXT  TYPE     CLUSTER   SIZE\n"
    )
    while line := p.recvline(keepends=False):
        pwn.log.info(f"LS: parsing line {line!r}\n")
        name_ext, typ, cluster, size = line.decode().lower().rsplit(None, 3)
        if typ == 'file':
            name, ext = name_ext.strip().rsplit(None, 1)
        else:
            name = name_ext
            ext = None
        content.append({
            'name': name.strip(),
            'ext': ext,
            'type': typ.strip().lower(),
            'cluster': int(cluster.strip()),
            'size': int(size.strip(), 16)
        })
    return content


def getroot(p: remote) -> PosRemote:
    p.recvuntil(b"You are in cluster ")
    p.pos = int(p.recvline())
    return p


def move_to(p: PosRemote, target: int):
    pwn.log.info(f'Moving from {p.pos} to {target}')
    for m in moves(p.pos, target):
        skipsection(p)
        p.send(m)
        p.recvuntil(b'---------------------------------------\n')
    p.pos = target


def explore_for(p: PosRemote, pred: Callable) -> Union[None, dict]:
    files = lsfiles(p)
    for f in files:
        if pred(f):
            return f
    for f in files:
        if f['type'] == 'folder' and f['name'] not in ('.', '..'):
            move_to(p, f['cluster'])
            found = explore_for(p, pred)
            if found:
                return found
    return None


def readfile(p: PosRemote, target: dict) -> bytes:
    dest = target['cluster']
    buf = []
    while dest < 0xFF8:
        move_to(p, dest)
        buf.append(bincontent(p))
        dest = getindex(p, dest)
    bs = b''.join(buf)
    if (len(bs) != target['size']):
        pwn.log.error(
            f"Couldn't get full file size for target {target}:\ngot {len(bs)} bytes [{bs}]"
        )
    return bs


def skipsection(p: PosRemote):
    # p.recvlines(timeout=1)
    read = p.recvuntil(b'\n\n', timeout=0.5)
    pwn.log.debug(f'Read {read!r} while skipping')


def getfile(p: remote, pred: Callable):
    try:
        p.recvline()
        p.send(b'lollol3\n')
        p.recvline()
        p.send(b'verduckz\n')
    except:
        pass
    p = getroot(p)
    target = explore_for(p, pred)
    if target is None:
        pwn.log.error(f'Target not found')
        return None
    pwn.log.success(f'Found target: {target}')
    return target, readfile(p, target)


def is_pic(f: dict):
    return f['ext'] in {'jpg', 'png', 'bmp', 'gif'}


pwn.log.setLevel(logging.DEBUG)
p = pwn.remote(ip, port)
result = getfile(p, is_pic)
if result is None:
    exit(1)
target, content = result
fname = target['name'] + '.' + (target['ext'] or '')
pwn.log.success(f'Read content of target, saving to {fname}')
with open(fname, 'wb') as f:
    f.write(content)
