# This code is the x86 entry point for 32->64bit calls.
# We stamp out a separate copy of it per entry point.

tramp64:
    # Argument passed to call64 trampoline:
    pushl $0x99999999 # hi 4 bytes of 64bit target
    pushl $0x11111111 # lo 4 bytes of 64bit target

    lcall *0xaaaaaaaa # 16:32 of call64

    # stack contents are now:
    #   8 bytes of 64bit target
    #   4 bytes return addr in exe
    #   N bytes arguments passed via stdcall

    retw $20  # clean stdcall args
