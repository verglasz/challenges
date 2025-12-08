.entrypoint
    // Code size       98 (0x62)
    .maxstack  3
    .locals init (int32[] V_0,
             int32 V_1,
             int32[] V_2,
             int32 V_3,
             int32 V_4)
    IL_0000:  ldc.i4.s   10
    IL_0002:  newarr     [System.Runtime]System.Int32
    IL_0007:  stloc.0
    IL_0008:  ldloc.0
    IL_0009:  ldc.i4.0
    IL_000a:  ldc.i4.0
    IL_000b:  stelem.i4
    IL_000c:  ldloc.0
    IL_000d:  ldc.i4.1
    IL_000e:  ldc.i4.2
    IL_000f:  stelem.i4
    IL_0010:  ldloc.0
    IL_0011:  ldc.i4.2
    IL_0012:  ldc.i4.s   -3
    IL_0014:  stelem.i4
    IL_0015:  ldloc.0
    IL_0016:  ldc.i4.3
    IL_0017:  ldc.i4.s   -11
    IL_0019:  stelem.i4
    IL_001a:  ldloc.0
    IL_001b:  ldc.i4.4
    IL_001c:  ldc.i4.s   17
    IL_001e:  stelem.i4
    IL_001f:  ldloc.0
    IL_0020:  ldc.i4.5
    IL_0021:  ldc.i4.s   -18
    IL_0023:  stelem.i4
    IL_0024:  ldloc.0
    IL_0025:  ldc.i4.6
    IL_0026:  ldc.i4.s   17
    IL_0028:  stelem.i4
    IL_0029:  ldloc.0
    IL_002a:  ldc.i4.7
    IL_002b:  ldc.i4.s   -11
    IL_002d:  stelem.i4
    IL_002e:  ldloc.0
    IL_002f:  ldc.i4.8
    IL_0030:  ldc.i4.s   13
    IL_0032:  stelem.i4
    IL_0033:  ldloc.0
    IL_0034:  ldc.i4.s   9
    IL_0036:  ldc.i4.s   -17
    IL_0038:  stelem.i4
    IL_0039:  ldc.i4.s   112
    IL_003b:  stloc.1
    IL_003c:  nop
    IL_003d:  ldloc.0
    IL_003e:  stloc.2
    IL_003f:  ldc.i4.0
    IL_0040:  stloc.3
    IL_0041:  br.s       IL_005b

    IL_0043:  ldloc.2
    IL_0044:  ldloc.3
    IL_0045:  ldelem.i4
    IL_0046:  stloc.s    V_4
    IL_0048:  nop
    IL_0049:  ldloc.1
    IL_004a:  ldloc.s    V_4
    IL_004c:  add
    IL_004d:  stloc.1
    IL_004e:  ldloc.1
    IL_004f:  conv.u2
    IL_0050:  call       void [System.Console]System.Console::Write(char)
    IL_0055:  nop
    IL_0056:  nop
    IL_0057:  ldloc.3
    IL_0058:  ldc.i4.1
    IL_0059:  add
    IL_005a:  stloc.3

    IL_005b:  ldloc.3
    IL_005c:  ldloc.2
    IL_005d:  ldlen
    IL_005e:  conv.i4
    IL_005f:  blt.s      IL_0043

    IL_0061:  ret

