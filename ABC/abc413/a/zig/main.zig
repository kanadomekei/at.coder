const std = @import("std");

const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

pub fn readLine(allocator: Allocator) ![]u8 {
    const stdin = std.io.getStdIn().reader();
    return try stdin.readUntilDelimiterAlloc(allocator, '\n', 1024 * 1024);
}

pub fn readInt(comptime T: type) !T {
    var buffer: [64]u8 = undefined;
    const stdin = std.io.getStdIn().reader();
    if (try stdin.readUntilDelimiterOrEof(buffer[0..], '\n')) |line| {
        const trimmed = std.mem.trim(u8, line, " \t\n\r");
        return std.fmt.parseInt(T, trimmed, 10);
    }
    return error.EndOfStream;
}

pub fn readInts(comptime T: type, allocator: Allocator) ![]T {
    const line = try readLine(allocator);
    defer allocator.free(line);

    var result = ArrayList(T).init(allocator);

    var iter = std.mem.splitSequence(u8, line, " ");
    while (iter.next()) |token| {
        const trimmed = std.mem.trim(u8, token, " \t\n\r");
        if (trimmed.len > 0) {
            const num = try std.fmt.parseInt(T, trimmed, 10);
            try result.append(num);
        }
    }

    return result.toOwnedSlice();
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const data = try readInts(i32, allocator);
    defer allocator.free(data);

    // const n = data[0];
    const m = data[1];

    const a = try readInts(i32, allocator);
    defer allocator.free(a);

    var sum: i32 = 0;

    for (a) |v| {
        sum += v;
    }

    if (sum <= m) {
        try std.io.getStdOut().writer().print("Yes\n", .{});
    } else {
        try std.io.getStdOut().writer().print("No\n", .{});
    }
}
