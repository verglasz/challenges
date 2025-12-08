.assembly extern System.Runtime
{
  .ver 6:0:0:0
  .publickeytoken = (B0 3F 5F 7F 11 D5 0A 3A ) // .?_....:
}
.assembly extern System.Console
{
  .ver 6:0:0:0
  .publickeytoken = (B0 3F 5F 7F 11 D5 0A 3A ) // .?_....:
}
.assembly 'decomplier'
{
  .custom instance void class [mscorlib]System.Runtime.CompilerServices.CompilationRelaxationsAttribute::'.ctor'(int32) =  (01 00 08 00 00 00 00 00 ) // ........

  .custom instance void class [mscorlib]System.Runtime.CompilerServices.RuntimeCompatibilityAttribute::'.ctor'() =  (
		01 00 01 00 54 02 16 57 72 61 70 4E 6F 6E 45 78   // ....T..WrapNonEx
		63 65 70 74 69 6F 6E 54 68 72 6F 77 73 01       ) // ceptionThrows.

  .custom instance void class [mscorlib]System.Diagnostics.DebuggableAttribute::'.ctor'(valuetype [mscorlib]System.Diagnostics.DebuggableAttribute/DebuggingModes) =  (01 00 02 00 00 00 00 00 ) // ........

  .custom instance void class [mscorlib]System.Runtime.Versioning.TargetFrameworkAttribute::'.ctor'(string) =  (
		01 00 18 2E 4E 45 54 43 6F 72 65 41 70 70 2C 56   // ....NETCoreApp,V
		65 72 73 69 6F 6E 3D 76 36 2E 30 01 00 54 0E 14   // ersion=v6.0..T..
		46 72 61 6D 65 77 6F 72 6B 44 69 73 70 6C 61 79   // FrameworkDisplay
		4E 61 6D 65 00                                  ) // Name.

  .custom instance void class [mscorlib]System.Reflection.AssemblyCompanyAttribute::'.ctor'(string) =  (01 00 0A 64 65 63 6F 6D 70 6C 69 65 72 00 00 ) // ...decomplier..

  .custom instance void class [mscorlib]System.Reflection.AssemblyConfigurationAttribute::'.ctor'(string) =  (01 00 07 52 65 6C 65 61 73 65 00 00 ) // ...Release..

  .custom instance void class [mscorlib]System.Reflection.AssemblyFileVersionAttribute::'.ctor'(string) =  (01 00 07 31 2E 30 2E 30 2E 30 00 00 ) // ...1.0.0.0..

  .custom instance void class [mscorlib]System.Reflection.AssemblyInformationalVersionAttribute::'.ctor'(string) =  (01 00 05 31 2E 30 2E 30 00 00 ) // ...1.0.0..

  .custom instance void class [mscorlib]System.Reflection.AssemblyProductAttribute::'.ctor'(string) =  (01 00 0A 64 65 63 6F 6D 70 6C 69 65 72 00 00 ) // ...decomplier..

  .custom instance void class [mscorlib]System.Reflection.AssemblyTitleAttribute::'.ctor'(string) =  (01 00 0A 64 65 63 6F 6D 70 6C 69 65 72 00 00 ) // ...decomplier..

  .hash algorithm 0x00008004
  .ver  1:0:0:0
}
.module decomplier.dll // GUID = {4E4DE787-C7B3-4A7D-A7DC-E49B331E62C0}


  .class private auto ansi beforefieldinit Program
  	extends [System.Runtime]System.Object
  {
    .custom instance void class [mscorlib]System.Runtime.CompilerServices.CompilerGeneratedAttribute::'.ctor'() =  (01 00 00 00 ) // ....


    // method line 1
    .method private static hidebysig
           default void '<Main>$' (string[] args)  cil managed
    {
        // Method begins at RVA 0x2050
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

    } // end of method Program::<Main>$

    // method line 2
    .method public hidebysig specialname rtspecialname
           instance default void '.ctor' ()  cil managed
    {
        // Method begins at RVA 0x2062
	// Code size 7 (0x7)
	.maxstack 8
	IL_0000:  ldarg.0
	IL_0001:  call instance void object::'.ctor'()
	IL_0006:  ret
    } // end of method Program::.ctor

  } // end of class Program

