const std = @import("std");

const clearFlags =
    \\pushw 0
    \\popf
    \\
;
const readFlags =
    \\pushf
    \\popw %[flags]
    \\
;

fn add(x: u16, y: u16) void {
    // XXX cannot have multiple outputs in zig
    const ret: u16 = asm (clearFlags ++
            \\movw %[x],%ax
            \\addw %[y],%ax
        ++ readFlags
        : [ret] "={ax}" (-> u16),
        : [x] "{ax}" (x),
          [y] "{bx}" (y),
          [flags] "{cx}" (flags),
    );
    std.debug.print("add({x},{x}) => {x}\n", .{ x, y, ret });
}

pub fn main() void {
    add(3, 4);
}
