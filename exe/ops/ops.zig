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
            const ret = asm (clearFlags ++ "\n" ++ code ++ "\n" ++ readFlags
                : [ret] "={eax}" (-> T),
                  [flags] "={cx}" (flags),
                : [x] "{eax}" (x),
                  [y] "{ebx}" (y),
            );
            std.debug.print(" => {x}", .{ret});
            printFlags(flags);
            std.debug.print("\n", .{});
        }
    }.f;
}

fn testAdd() void {
    const add32 = genAsm("add", u32, "addl %[y],%[x]");
    const t: i32 = -3;
    add32(3, @bitCast(u32, t));
    add32(0xFFFF_FFFF, 0);
    add32(0xFFFF_FFFF, 1);
}

pub fn main() void {
    testAdd();
}
