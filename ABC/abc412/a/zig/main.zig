const std = @import("std");

const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;

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

pub fn read2DInts(comptime T: type, allocator: Allocator, n: usize) ![][]T {
    var result = ArrayList([]T).init(allocator);
    errdefer {
        for (result.items) |row| {
            allocator.free(row);
        }
        result.deinit();
    }

    var i: usize = 0;
    while (i < n) : (i += 1) {
        const row = try readInts(T, allocator);
        try result.append(row);
    }

    return result.toOwnedSlice();
}

pub fn free2DSlice(comptime T: type, allocator: Allocator, slice: [][]T) void {
    for (slice) |row| {
        allocator.free(row);
    }
    allocator.free(slice);
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const n = try readInt(usize);

    const data = try read2DInts(i32, allocator, n);
    defer free2DSlice(i32, allocator, data);

    var count: i32 = 0;
    for (data) |day| {
        const goal = day[0];
        const actual = day[1];
        if (actual > goal) {
            count += 1;
        }
    }

    try std.io.getStdOut().writer().print("{d}\n", .{count});
}
