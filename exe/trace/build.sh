#!/bin/sh

# Use cpu=i686 here to avoid generating SSE instructions.

opt="-O ReleaseSmall"
exec zig build-exe trace.zig -mcpu=i686 $opt -target x86-windows-msvc -fsingle-threaded "$@"
