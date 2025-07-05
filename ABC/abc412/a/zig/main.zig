const std = @import("std");
const print = std.debug.print;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

pub fn readInt(comptime T: type) !T {
    var buffer: [64]u8 = undefined;
    const stdin = std.io.getStdIn().reader();
    if (try stdin.readUntilDelimiterOrEof(buffer[0..], '\n')) |line| {
        const trimmed = std.mem.trim(u8, line, " \t\n\r");
        return std.fmt.parseInt(T, trimmed, 10);
    }
    return error.EndOfStream;
}

pub fn main() !void {
    const n = try readInt(i32);

    std.debug.print("{}\n", .{n});
}
