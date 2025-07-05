const std = @import("std");

const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    // const allocator = gpa.allocator();

    try std.io.getStdOut().writer().print("Hello, {s}!\n", .{"World"});
}
