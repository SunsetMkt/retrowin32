#!/bin/sh

#  

export RUSTFLAGS='--print link-args -C relocation-model=dynamic-no-pic '

for link_arg in -Wl,-no_fixup_chains -Wl,-no_pie -pagezero_size 0x4000 -image_base 0x10000000 -segaddr XYZ 0x4000; do
    RUSTFLAGS="$RUSTFLAGS -C link_arg=$link_arg"
done

# -C relocation-model=dynamic-no-pic -C link_arg=-Wl,-no_fixup_chains -C link-arg=-Wl,-no_pie -C link-arg=-pagezero_size -C link-arg=0x4000 -C link-arg=-image_base -C link-arg=0x10000000 -C link-arg=-segaddr -C link-arg=XYZ -C link-arg=0x4000'
# -C link-arg=-seg_page_size -C link-arg=WINE_4GB_RESERVE -C link-arg=0x100000'
exec cargo build --target x86_64-apple-darwin -p retrowin32 --no-default-features -v
