# This code is the x86 entry point for 32->64bit calls.
# We stamp out a separate copy of it per entry point.

tramp64:
    # We use 'far call', opcode FF
    # https://www.felixcloutier.com/x86/call
    # It needs target in memory as 16:32 selector:addr, which we push on the stack:
    pushl $0x2b  # selector, todo do not hardcode this
    pushl $0xAAAAAAAA # this will be the addr of call64

    # Argument passed to that trampoline:
    pushl $0x99999999 # hi 4 bytes of 64bit target
    pushl $0x11111111 # lo 4 bytes of 64bit target

    lcalll *8(%esp)

    addl $0x10, %esp

    # XXX caller came here expecting stdcall, so we need to clean its stack args,
    # dependent on which function we're trampolining
    retl 20
