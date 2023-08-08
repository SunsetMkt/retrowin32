https://www.winehq.org/pipermail/wine-devel/2019-December/156602.html

> Speaking of code segments, the big thing that Catalina provides that enables
> this all to work is the ability to create 32-bit code segments in a 64-bit
> process. For that, they enabled the use of i386_set_ldt() in 64-bit processes.
> The big caveat though, is that this functionality is restricted by System
> Integrity Protection (SIP). For now, your best bet to get this working for
> yourself is to disable SIP. (CrossOver doesn't require that, but the mechanism
> by which we accomplish that is in flux internally to Apple. When it settles
> down, I'll update this thread.)

https://stackoverflow.com/questions/53244454/how-did-wine64-manage-to-handle-macos

https://github.com/apple/darwin-xnu/blob/main/tests/ldt.c#L73

we

wine: signal_init_process in signal_x86_64 sets up ldt (?)

32-bit mapping:
https://stackoverflow.com/questions/69791314/cannot-create-anonymous-mapping-with-map-32bit-on-macos

loader: On Mac, reserve an area starting at 4GB to force Rosetta's allocations
higher.ld https://www.winehq.org/pipermail/wine-cvs/2021-June/154645.html

link-arg instead of link-args for pagezero_size

long call
https://github.com/llvm/llvm-project/blob/ef888bc67c726deb8c74ea32e7c8c9ace756b667/llvm/lib/Target/X86/X86InstrAsmAlias.td#L446

## 32->64 and back

In the exe, imagine some C code like

```
WriteFile(stdout, 0x1234);
```

in asm it's:

```
pushl 1234
pushl stdout
call WriteFile
```

where we choose the WriteFile jmp target when we load the PE.

So after the call is hit, we're in code we control, the stack looks like this
(top of stack at bottom):

```
1234
stdout
exe-return-addr
```

and our goal is to call into 64-bit code and back.

To do this, we make this jmp to code that looks like

```
push 0xAAAA_AAAA
push 0xBBBB_BBBB
ljmp *shim64
```

where 0xAAAA_AAAA_BBBB_BBBB is the 64-bit address of our (Rust) WriteFile
wrapper, and shim64 is some memory we set up ahead of time that contains the
m16:32 needed to switch segments: segment selector for 64-bit code : call64 addr

call64 is some 64-bit code we set up ahead of time, which looks like:

```
subl $8, %rsp
jmp *(%rsp+$8)
```

That snips the AAA/BBB off the stack and jmps to the wrapper.

When we get to the wrapper, the stack looks like the original stack picture. The
wrapper extracts the (callee-specific args, e.g. stdout etc. for WriteFile) and
calls our actual WriteFile implementation.

When that returns, we're back in our wrapper and we want to return to 32-bit
mode in the exe.

To do so, we need to:

1. clean the caller-cleared args
2. jmp to a m16:32 address for it.

We can do both by writing the m16: part of the destination address right over
exe-return-addr, leaving the stack like

```
1234
32-bit segment selector
exe-return-addr
```

and then executing asm like:

```
lret 12
```

which both cleans the stack _and_ does the segment swap.
