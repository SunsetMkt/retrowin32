const std = @import("std");

const clearFlags =
    \\pushl $0
    \\popfl
;
const readFlags =
    \\pushfw
    \\popw %[flags]
;

fn printFlags(flags: u16) void {
    if ((flags & 1) != 0)
        std.debug.print(" CF", .{});
    if ((flags & (1 << 6)) != 0)
        std.debug.print(" ZF", .{});
    if ((flags & (1 << 7)) != 0)
        std.debug.print(" SF", .{});
    if ((flags & (1 << 10)) != 0)
        std.debug.print(" DF", .{});
    if ((flags & (1 << 11)) != 0)
        std.debug.print(" OF", .{});
}

fn genAsm(comptime desc: []const u8, comptime T: type, comptime code: []const u8) fn (T, T) void {
    return struct {
        pub fn f(x: T, y: T) void {
            std.debug.print(desc ++ "({x},{x})", .{ x, y });
            var flags: u16 = 0;
            switch (T) {
                u32 => {
                    const ret = asm (clearFlags ++ "\n" ++ code ++ "\n" ++ readFlags
                        : [ret] "={eax}" (-> u32),
                          [flags] "={cx}" (flags),
                        : [x] "{eax}" (x),
                          [y] "{ebx}" (y),
                    );
                    std.debug.print(" => {x}", .{ret});
                },
                else => unreachable,
            }
            printFlags(flags);
            std.debug.print("\n", .{});
        }
    }.f;
}

fn testAdd() void {
    const add32 = genAsm("add", u32, "addl %[y],%[x]");
    add32(3, 4);
    add32(0xFFFF_FFFF, 0);
    add32(0xFFFF_FFFF, 1);
    const add8 = genAsm("add", u8, "addb %[y],%[x]");
    add8(3, 4);
}

pub fn main() void {
    testAdd();
}
