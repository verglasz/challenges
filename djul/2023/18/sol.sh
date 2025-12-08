#!/usr/bin/env sh


# fq ' .gap0 | tobytes ' ./duck.elf.jpg > duck.elf
llvm-objdump duck.elf --disassemble-symbols=main > duck.armel.asm
rg -o 'mov\s+r2, #(\d+)' duck.armel.asm  --replace='$1' |
	tr '\n' ' ' |
	python -c 'print(bytes(int(n) for n in input().split() ).decode())'

