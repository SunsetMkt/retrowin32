const print = @import("std").debug.print;

fn printFlags(flags: u16) void {
    if ((flags & 1) != 0)
        print(" CF", .{});
    if ((flags & (1 << 6)) != 0)
        print(" ZF", .{});
    if ((flags & (1 << 7)) != 0)
        print(" SF", .{});
    if ((flags & (1 << 10)) != 0)
        print(" DF", .{});
    if ((flags & (1 << 11)) != 0)
        print(" OF", .{});
}

fn asmFn(comptime desc: []const u8, comptime T: type, comptime code: []const u8) fn (T, T) void {
    const fullCode =
        \\pushl $0
        \\popfl
        \\
    ++ code ++ "\n" ++
        \\pushfw
        \\popw %[flags]
    ;
    return struct {
        pub fn f(x: T, y: T) void {
            print(desc ++ " {x},{x}", .{ x, y });
            var flags: u16 = 0;
            // We have to write out the asm block for each type because we cannot vary
            // the {eax} bits via comptime strings. :~(
            switch (T) {
                u32 => {
                    const ret = asm (fullCode
                        : [ret] "={eax}" (-> u32),
                          [flags] "={cx}" (flags),
                        : [x] "{eax}" (x),
                          [y] "{ebx}" (y),
                    );
                    print(" => {x}", .{ret});
                },
                u8 => {
                    const ret = asm (fullCode
                        : [ret] "={eax}" (-> u32),
                          [flags] "={cx}" (flags),
                        : [x] "{al}" (x),
                          [y] "{bl}" (y),
                    );
                    print(" => {x}", .{ret});
                },
                else => unreachable,
            }
            printFlags(flags);
            print("\n", .{});
        }
    }.f;
}

fn testAdd() void {
    const add8 = asmFn("add", u8, "addb %[y],%[x]");
    add8(3, 5);
    add8(3, 0xfd);
    add8(3, 0xfb);

    const adc = asmFn("adc (CF=1)", u8, "stc\nadcb %[y],%[x]");
    adc(0xff, 0);
    adc(0xff, 1);
    adc(0xff, 0xfe);
    adc(0xff, 0xff);
}

pub fn main() void {
    testAdd();
}
