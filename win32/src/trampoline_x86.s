# This code is the x86 entry point for 32->64bit calls.
# We stamp out a separate copy of it per entry point.

tramp64:
    # Argument passed to call64 trampoline:
    pushl $0x99999999 # hi 4 bytes of 64bit target
    pushl $0x11111111 # lo 4 bytes of 64bit target

    lcalll *0xaaaaaaaa # 16:32 of call64

    addl $0x8, %esp

    # XXX caller came here expecting stdcall, so we need to clean its stack args,
    # dependent on which function we're trampolining
    retl $0x20
