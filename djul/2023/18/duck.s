
duck.elf:	file format elf32-littlearm

Disassembly of section .init:

00010390 <_init>:
   10390: e92d4008     	push	{r3, lr}
   10394: eb000028     	bl	0x1043c <call_weak_fn>  @ imm = #0xa0

00010398 <$a>:
   10398: e8bd8008     	pop	{r3, pc}

Disassembly of section .plt:

0001039c <$a>:
   1039c: e52de004     	str	lr, [sp, #-0x4]!
   103a0: e59fe004     	ldr	lr, [pc, #0x4]          @ 0x103ac <.plt+0x10>
   103a4: e08fe00e     	add	lr, pc, lr
   103a8: e5bef008     	ldr	pc, [lr, #0x8]!

000103ac <$d>:
   103ac: 54 0c 01 00  	.word	0x00010c54

000103b0 <$a>:
   103b0: e28fc600     	add	r12, pc, #0, #12
   103b4: e28cca10     	add	r12, r12, #16, #20
   103b8: e5bcfc54     	ldr	pc, [r12, #0xc54]!
   103bc: e28fc600     	add	r12, pc, #0, #12
   103c0: e28cca10     	add	r12, r12, #16, #20
   103c4: e5bcfc4c     	ldr	pc, [r12, #0xc4c]!
   103c8: e28fc600     	add	r12, pc, #0, #12
   103cc: e28cca10     	add	r12, r12, #16, #20
   103d0: e5bcfc44     	ldr	pc, [r12, #0xc44]!
   103d4: e28fc600     	add	r12, pc, #0, #12
   103d8: e28cca10     	add	r12, r12, #16, #20
   103dc: e5bcfc3c     	ldr	pc, [r12, #0xc3c]!
   103e0: e28fc600     	add	r12, pc, #0, #12
   103e4: e28cca10     	add	r12, r12, #16, #20
   103e8: e5bcfc34     	ldr	pc, [r12, #0xc34]!
   103ec: e28fc600     	add	r12, pc, #0, #12
   103f0: e28cca10     	add	r12, r12, #16, #20
   103f4: e5bcfc2c     	ldr	pc, [r12, #0xc2c]!

Disassembly of section .text:

000103f8 <_start>:
   103f8: e3a0b000     	mov	r11, #0
   103fc: e3a0e000     	mov	lr, #0
   10400: e49d1004     	ldr	r1, [sp], #4
   10404: e1a0200d     	mov	r2, sp
   10408: e52d2004     	str	r2, [sp, #-0x4]!
   1040c: e52d0004     	str	r0, [sp, #-0x4]!
   10410: e59fa01c     	ldr	r10, [pc, #0x1c]        @ 0x10434 <_start+0x3c>
   10414: e28f3018     	add	r3, pc, #24
   10418: e08aa003     	add	r10, r10, r3
   1041c: e3a03000     	mov	r3, #0
   10420: e52d3004     	str	r3, [sp, #-0x4]!
   10424: e59f000c     	ldr	r0, [pc, #0xc]          @ 0x10438 <_start+0x40>
   10428: e79a0000     	ldr	r0, [r10, r0]
   1042c: ebffffdf     	bl	0x103b0 <.plt+0x14>     @ imm = #-0x84
   10430: ebffffed     	bl	0x103ec <.plt+0x50>     @ imm = #-0x4c

00010434 <$d>:
   10434: cc 0b 01 00  	.word	0x00010bcc
   10438: 28 00 00 00  	.word	0x00000028

0001043c <call_weak_fn>:
   1043c: e59f3014     	ldr	r3, [pc, #0x14]         @ 0x10458 <call_weak_fn+0x1c>
   10440: e59f2014     	ldr	r2, [pc, #0x14]         @ 0x1045c <call_weak_fn+0x20>
   10444: e08f3003     	add	r3, pc, r3
   10448: e7932002     	ldr	r2, [r3, r2]
   1044c: e3520000     	cmp	r2, #0
   10450: 012fff1e     	bxeq	lr
   10454: eaffffde     	b	0x103d4 <.plt+0x38>     @ imm = #-0x88

00010458 <$d>:
   10458: b4 0b 01 00  	.word	0x00010bb4
   1045c: 24 00 00 00  	.word	0x00000024

00010460 <deregister_tm_clones>:
   10460: e59f0018     	ldr	r0, [pc, #0x18]         @ 0x10480 <deregister_tm_clones+0x20>
   10464: e59f3018     	ldr	r3, [pc, #0x18]         @ 0x10484 <deregister_tm_clones+0x24>
   10468: e1530000     	cmp	r3, r0
   1046c: 012fff1e     	bxeq	lr
   10470: e59f3010     	ldr	r3, [pc, #0x10]         @ 0x10488 <deregister_tm_clones+0x28>
   10474: e3530000     	cmp	r3, #0
   10478: 012fff1e     	bxeq	lr
   1047c: e12fff13     	bx	r3

00010480 <$d>:
   10480: 34 10 02 00  	.word	0x00021034
   10484: 34 10 02 00  	.word	0x00021034
   10488: 00 00 00 00  	.word	0x00000000

0001048c <register_tm_clones>:
   1048c: e59f0024     	ldr	r0, [pc, #0x24]         @ 0x104b8 <register_tm_clones+0x2c>
   10490: e59f3024     	ldr	r3, [pc, #0x24]         @ 0x104bc <register_tm_clones+0x30>
   10494: e0433000     	sub	r3, r3, r0
   10498: e1a01fa3     	lsr	r1, r3, #31
   1049c: e0811143     	add	r1, r1, r3, asr #2
   104a0: e1b010c1     	asrs	r1, r1, #1
   104a4: 012fff1e     	bxeq	lr
   104a8: e59f3010     	ldr	r3, [pc, #0x10]         @ 0x104c0 <register_tm_clones+0x34>
   104ac: e3530000     	cmp	r3, #0
   104b0: 012fff1e     	bxeq	lr
   104b4: e12fff13     	bx	r3

000104b8 <$d>:
   104b8: 34 10 02 00  	.word	0x00021034
   104bc: 34 10 02 00  	.word	0x00021034
   104c0: 00 00 00 00  	.word	0x00000000

000104c4 <__do_global_dtors_aux>:
   104c4: e92d4010     	push	{r4, lr}
   104c8: e59f4018     	ldr	r4, [pc, #0x18]         @ 0x104e8 <__do_global_dtors_aux+0x24>
   104cc: e5d43000     	ldrb	r3, [r4]
   104d0: e3530000     	cmp	r3, #0
   104d4: 18bd8010     	popne	{r4, pc}
   104d8: ebffffe0     	bl	0x10460 <deregister_tm_clones> @ imm = #-0x80
   104dc: e3a03001     	mov	r3, #1
   104e0: e5c43000     	strb	r3, [r4]
   104e4: e8bd8010     	pop	{r4, pc}

000104e8 <$d>:
   104e8: 34 10 02 00  	.word	0x00021034

000104ec <frame_dummy>:
   104ec: eaffffe6     	b	0x1048c <register_tm_clones> @ imm = #-0x68

000104f0 <w0rk>:
   104f0: e92d4800     	push	{r11, lr}
   104f4: e28db004     	add	r11, sp, #4
   104f8: e59f0010     	ldr	r0, [pc, #0x10]         @ 0x10510 <w0rk+0x20>
   104fc: ebffffb1     	bl	0x103c8 <.plt+0x2c>     @ imm = #-0x13c
   10500: e59f000c     	ldr	r0, [pc, #0xc]          @ 0x10514 <w0rk+0x24>
   10504: ebffffaf     	bl	0x103c8 <.plt+0x2c>     @ imm = #-0x144
   10508: e1a00000     	mov	r0, r0
   1050c: e8bd8800     	pop	{r11, pc}

00010510 <$d>:
   10510: 10 08 01 00  	.word	0x00010810
   10514: 38 08 01 00  	.word	0x00010838

00010518 <main>:
   10518: e92d4800     	push	{r11, lr}
   1051c: e28db004     	add	r11, sp, #4
   10520: e24dd020     	sub	sp, sp, #32
   10524: e59f3244     	ldr	r3, [pc, #0x244]        @ 0x10770 <main+0x258>
   10528: e5933000     	ldr	r3, [r3]
   1052c: e50b3008     	str	r3, [r11, #-0x8]
   10530: e3a03000     	mov	r3, #0
   10534: e3a02066     	mov	r2, #102
   10538: e3a0300e     	mov	r3, #14
   1053c: e0233002     	eor	r3, r3, r2
   10540: e20330ff     	and	r3, r3, #255
   10544: e1a00003     	mov	r0, r3
   10548: ebffffa4     	bl	0x103e0 <.plt+0x44>     @ imm = #-0x170
   1054c: e3a0206c     	mov	r2, #108
   10550: e3a0305c     	mov	r3, #92
   10554: e0233002     	eor	r3, r3, r2
   10558: e20330ff     	and	r3, r3, #255
   1055c: e1a00003     	mov	r0, r3
   10560: ebffff9e     	bl	0x103e0 <.plt+0x44>     @ imm = #-0x188
   10564: e3a02061     	mov	r2, #97
   10568: e3a03051     	mov	r3, #81
   1056c: e0233002     	eor	r3, r3, r2
   10570: e20330ff     	and	r3, r3, #255
   10574: e1a00003     	mov	r0, r3
   10578: ebffff98     	bl	0x103e0 <.plt+0x44>     @ imm = #-0x1a0
   1057c: e3a02067     	mov	r2, #103
   10580: e3a03057     	mov	r3, #87
   10584: e0233002     	eor	r3, r3, r2
   10588: e20330ff     	and	r3, r3, #255
   1058c: e1a00003     	mov	r0, r3
   10590: ebffff92     	bl	0x103e0 <.plt+0x44>     @ imm = #-0x1b8
   10594: e3a0203a     	mov	r2, #58
   10598: e3a03052     	mov	r3, #82
   1059c: e0233002     	eor	r3, r3, r2
   105a0: e20330ff     	and	r3, r3, #255
   105a4: e1a00003     	mov	r0, r3
   105a8: ebffff8c     	bl	0x103e0 <.plt+0x44>     @ imm = #-0x1d0
   105ac: e3a02067     	mov	r2, #103
   105b0: e3a03057     	mov	r3, #87
   105b4: e0233002     	eor	r3, r3, r2
   105b8: e20330ff     	and	r3, r3, #255
   105bc: e1a00003     	mov	r0, r3
   105c0: ebffff86     	bl	0x103e0 <.plt+0x44>     @ imm = #-0x1e8
   105c4: e3a02030     	mov	r2, #48
   105c8: e3a03000     	mov	r3, #0
   105cc: e0233002     	eor	r3, r3, r2
   105d0: e20330ff     	and	r3, r3, #255
   105d4: e1a00003     	mov	r0, r3
   105d8: ebffff80     	bl	0x103e0 <.plt+0x44>     @ imm = #-0x200
   105dc: e3a02064     	mov	r2, #100
   105e0: e3a0303b     	mov	r3, #59
   105e4: e0233002     	eor	r3, r3, r2
   105e8: e20330ff     	and	r3, r3, #255
   105ec: e1a00003     	mov	r0, r3
   105f0: ebffff7a     	bl	0x103e0 <.plt+0x44>     @ imm = #-0x218
   105f4: e3a0205f     	mov	r2, #95
   105f8: e3a03032     	mov	r3, #50
   105fc: e0233002     	eor	r3, r3, r2
   10600: e20330ff     	and	r3, r3, #255
   10604: e1a00003     	mov	r0, r3
   10608: ebffff74     	bl	0x103e0 <.plt+0x44>     @ imm = #-0x230
   1060c: e3a0206a     	mov	r2, #106
   10610: e3a0305e     	mov	r3, #94
   10614: e0233002     	eor	r3, r3, r2
   10618: e20330ff     	and	r3, r3, #255
   1061c: e1a00003     	mov	r0, r3
   10620: ebffff6e     	bl	0x103e0 <.plt+0x44>     @ imm = #-0x248
   10624: e3a02075     	mov	r2, #117
   10628: e3a03007     	mov	r3, #7
   1062c: e0233002     	eor	r3, r3, r2
   10630: e20330ff     	and	r3, r3, #255
   10634: e1a00003     	mov	r0, r3
   10638: ebffff68     	bl	0x103e0 <.plt+0x44>     @ imm = #-0x260
   1063c: e3a02031     	mov	r2, #49
   10640: e3a03043     	mov	r3, #67
   10644: e0233002     	eor	r3, r3, r2
   10648: e20330ff     	and	r3, r3, #255
   1064c: e1a00003     	mov	r0, r3
   10650: ebffff62     	bl	0x103e0 <.plt+0x44>     @ imm = #-0x278
   10654: e3a0205f     	mov	r2, #95
   10658: e3a03026     	mov	r3, #38
   1065c: e0233002     	eor	r3, r3, r2
   10660: e20330ff     	and	r3, r3, #255
   10664: e1a00003     	mov	r0, r3
   10668: ebffff5c     	bl	0x103e0 <.plt+0x44>     @ imm = #-0x290
   1066c: e3a02066     	mov	r2, #102
   10670: e3a03039     	mov	r3, #57
   10674: e0233002     	eor	r3, r3, r2
   10678: e20330ff     	and	r3, r3, #255
   1067c: e1a00003     	mov	r0, r3
   10680: ebffff56     	bl	0x103e0 <.plt+0x44>     @ imm = #-0x2a8
   10684: e3a02072     	mov	r2, #114
   10688: e3a03016     	mov	r3, #22
   1068c: e0233002     	eor	r3, r3, r2
   10690: e20330ff     	and	r3, r3, #255
   10694: e1a00003     	mov	r0, r3
   10698: ebffff50     	bl	0x103e0 <.plt+0x44>     @ imm = #-0x2c0
   1069c: e3a02030     	mov	r2, #48
   106a0: e3a03001     	mov	r3, #1
   106a4: e0233002     	eor	r3, r3, r2
   106a8: e20330ff     	and	r3, r3, #255
   106ac: e1a00003     	mov	r0, r3
   106b0: ebffff4a     	bl	0x103e0 <.plt+0x44>     @ imm = #-0x2d8
   106b4: e3a0206d     	mov	r2, #109
   106b8: e3a0301e     	mov	r3, #30
   106bc: e0233002     	eor	r3, r3, r2
   106c0: e20330ff     	and	r3, r3, #255
   106c4: e1a00003     	mov	r0, r3
   106c8: ebffff44     	bl	0x103e0 <.plt+0x44>     @ imm = #-0x2f0
   106cc: e3a0205f     	mov	r2, #95
   106d0: e3a03032     	mov	r3, #50
   106d4: e0233002     	eor	r3, r3, r2
   106d8: e20330ff     	and	r3, r3, #255
   106dc: e1a00003     	mov	r0, r3
   106e0: ebffff3e     	bl	0x103e0 <.plt+0x44>     @ imm = #-0x308
   106e4: e3a0206d     	mov	r2, #109
   106e8: e3a0300c     	mov	r3, #12
   106ec: e0233002     	eor	r3, r3, r2
   106f0: e20330ff     	and	r3, r3, #255
   106f4: e1a00003     	mov	r0, r3
   106f8: ebffff38     	bl	0x103e0 <.plt+0x44>     @ imm = #-0x320
   106fc: e3a02073     	mov	r2, #115
   10700: e3a03000     	mov	r3, #0
   10704: e0233002     	eor	r3, r3, r2
   10708: e20330ff     	and	r3, r3, #255
   1070c: e1a00003     	mov	r0, r3
   10710: ebffff32     	bl	0x103e0 <.plt+0x44>     @ imm = #-0x338
   10714: e3a02034     	mov	r2, #52
   10718: e3a03047     	mov	r3, #71
   1071c: e0233002     	eor	r3, r3, r2
   10720: e20330ff     	and	r3, r3, #255
   10724: e1a00003     	mov	r0, r3
   10728: ebffff2c     	bl	0x103e0 <.plt+0x44>     @ imm = #-0x350
   1072c: e3a02062     	mov	r2, #98
   10730: e3a03011     	mov	r3, #17
   10734: e0233002     	eor	r3, r3, r2
   10738: e20330ff     	and	r3, r3, #255
   1073c: e1a00003     	mov	r0, r3
   10740: ebffff26     	bl	0x103e0 <.plt+0x44>     @ imm = #-0x368
   10744: e3a03000     	mov	r3, #0
   10748: e59f2020     	ldr	r2, [pc, #0x20]         @ 0x10770 <main+0x258>
   1074c: e5921000     	ldr	r1, [r2]
   10750: e51b2008     	ldr	r2, [r11, #-0x8]
   10754: e0321001     	eors	r1, r2, r1
   10758: e3a02000     	mov	r2, #0
   1075c: 0a000000     	beq	0x10764 <main+0x24c>    @ imm = #0x0
   10760: ebffff15     	bl	0x103bc <.plt+0x20>     @ imm = #-0x3ac
   10764: e1a00003     	mov	r0, r3
   10768: e24bd004     	sub	sp, r11, #4
   1076c: e8bd8800     	pop	{r11, pc}

00010770 <$d>:
   10770: 08 0f 02 00  	.word	0x00020f08

Disassembly of section .fini:

00010774 <_fini>:
   10774: e92d4008     	push	{r3, lr}

00010778 <$a>:
   10778: e8bd8008     	pop	{r3, pc}
