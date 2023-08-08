#!/bin/sh

set -e -o pipefail

clang -c -target i386-apple-darwin win32/src/trampoline_x86.s -o - | objdump -D -

clang -c -target x86_64-apple-darwin win32/src/trampoline_x86_64.s -o - | objdump -D -
