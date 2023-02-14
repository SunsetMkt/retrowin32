#!/bin/sh

exec zig build-exe ops.zig -O ReleaseSmall -target x86-windows -fsingle-threaded
