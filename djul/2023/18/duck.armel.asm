
duck.elf:	file format elf32-littlearm

Disassembly of section .text:

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
