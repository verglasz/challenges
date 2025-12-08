#!/usr/bin/env python3
s1 = "79776f6473737964627f67756864"
s2 = s1[1:] + s1[0]
s3 = s1[::-1]
s4 = s3[1:] + s3[0]
b1 = bytes.fromhex(s1)
b2 = bytes.fromhex(s2)
b3 = bytes.fromhex(s3)
b4 = bytes.fromhex(s4)
print(b1)
print(b2)
print(b3)
print(b4)
