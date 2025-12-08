this was a pain in the ass

lots of great solutions for reversing C#! a shame they're all for windows! god fucking dammit

tried to install dotPeek (apparently the best) with wine but kept failing (cleaning wineprefix and
reinstalling dotnet with winetricks didn't help etc), then i tried to use other tools
(including plain visual studio ildasm/ilasm) but still
couldn't get to run them under wine, then found ILSpy for linux with a broken aur package
(`ilspymono-git`) and a working one (`avaloniailspy`), this last one actually worked and
managed to decompile to C# but actually would've failed the challenge if i didn't first run
the dll through `monodis` (mono's equivalent to `ildasm` i guess which i magically found out about
(don't remember how)) since there's a string that's pushed and popped which
`ilspy` doesn't get (it doesn't show it anywhere) and that string contains
more IL code (ICL? MSIL? whatever, the C# vm assembly repr of bytecode i guess).
that IL code seems to be a method body, so i pasted it in place of the body of `Main` in the
original IL disassembly then since there's no IL -> C# decompiler i could find i just assembled it
with mono's `ilasm` and ran it through ilspy again, which finally produced a working method body
which i compiled again with mono's `mcs` and ran through wine (since the actual executable didn't
ran through wine), though i later realized both of them simply run through `mono` directly...

well, if it worked it worked.
