import hashlib
import base64

# did you know the key is 32 bytes following the header '== ed25519v1-public: type0 ==\x00\x00\x00'????
# NEITHER DID I
with open('key_bytes', 'rb') as f:
    key = f.read()

# did you know onion V3 addresses are the b32 encoding of <pubkey> + <checksum> + \x03?
# and that the checksum is the first 2 bytes of SPECIFICALLY the sha3_256 hash of
# '.onion checksum' + <pubkey> + \x03? well neither did i, and how the fuck this info is nowhere!!
# best i found is this page that doesn't specify the hash used:
# https://gitweb.torproject.org/torspec.git/tree/rend-spec-v3.txt
# it links to something that says SHA3 but not 256.......
to_chsum = b'%b%b%b' % (b'.onion checksum', key, b'\x03')
checksum = hashlib.sha3_256(to_chsum).digest()
to_addr = b'%b%b%b' % (key, checksum[:2], b'\x03')
address = base64.b32encode(to_addr).decode()
print(address + '.onion')
