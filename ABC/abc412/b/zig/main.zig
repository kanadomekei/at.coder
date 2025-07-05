const std = @import("std");

const Allocator = std.mem.Allocator;
const AutoHashMap = std.AutoHashMap;

pub fn readLine(allocator: Allocator) ![]u8 {
    const stdin = std.io.getStdIn().reader();
    return try stdin.readUntilDelimiterAlloc(allocator, '\n', 1 * 1024 * 1024);
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const s = try readLine(allocator);
    defer allocator.free(s);
    const t = try readLine(allocator);
    defer allocator.free(t);

    // tの文字をハッシュセット（HashMap<u8, void>）に入れる
    var t_set = AutoHashMap(u8, void).init(allocator);
    defer t_set.deinit();
    for (t) |char_t| {
        try t_set.put(char_t, {});
    }

    for (1..s.len) |i| {
        const char = s[i];
        const prev_char = s[i - 1];
        if (std.ascii.isUpper(char)) {
            // O(1)のハッシュセット検索に置き換える
            if (!t_set.contains(prev_char)) {
                try std.io.getStdOut().writer().print("No\n", .{});
                return;
            }
        }
    }

    try std.io.getStdOut().writer().print("Yes\n", .{});
}
