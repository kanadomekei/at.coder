const std = @import("std");
const print = std.debug.print;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

pub fn readLine(allocator: Allocator) ![]u8 {
    const stdin = std.io.getStdIn().reader();
    return try stdin.readUntilDelimiterAlloc(allocator, '\n', 1024);
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
    defer result.deinit();

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

pub fn readString(allocator: Allocator) ![]u8 {
    return readLine(allocator);
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

pub fn max(comptime T: type, a: T, b: T) T {
    return if (a > b) a else b;
}

pub fn min(comptime T: type, a: T, b: T) T {
    return if (a < b) a else b;
}

pub fn abs(comptime T: type, x: T) T {
    return if (x < 0) -x else x;
}

pub fn pow(comptime T: type, base: T, exp: T) T {
    if (exp == 0) return 1;
    var result: T = 1;
    var b = base;
    var e = exp;
    while (e > 0) {
        if (e % 2 == 1) {
            result *= b;
        }
        b *= b;
        e /= 2;
    }
    return result;
}

pub fn gcd(comptime T: type, a: T, b: T) T {
    var x = a;
    var y = b;
    while (y != 0) {
        const temp = y;
        y = @rem(x, y);
        x = temp;
    }
    return x;
}

pub fn lcm(comptime T: type, a: T, b: T) T {
    return a / gcd(T, a, b) * b;
}

pub fn reverseSlice(comptime T: type, arr: []T) void {
    var i: usize = 0;
    var j: usize = arr.len - 1;
    while (i < j) {
        const temp = arr[i];
        arr[i] = arr[j];
        arr[j] = temp;
        i += 1;
        j -= 1;
    }
}

pub fn sumSlice(comptime T: type, arr: []const T) T {
    var sum: T = 0;
    for (arr) |item| {
        sum += item;
    }
    return sum;
}

pub fn maxSlice(comptime T: type, arr: []const T) T {
    if (arr.len == 0) return 0;
    var maximum = arr[0];
    for (arr[1..]) |item| {
        if (item > maximum) {
            maximum = item;
        }
    }
    return maximum;
}

pub fn minSlice(comptime T: type, arr: []const T) T {
    if (arr.len == 0) return 0;
    var minimum = arr[0];
    for (arr[1..]) |item| {
        if (item < minimum) {
            minimum = item;
        }
    }
    return minimum;
}

pub fn isPrime(n: i64) bool {
    if (n < 2) return false;
    if (n == 2) return true;
    if (@rem(n, 2) == 0) return false;

    var i: i64 = 3;
    while (i * i <= n) {
        if (@rem(n, i) == 0) return false;
        i += 2;
    }
    return true;
}

pub fn sieveOfEratosthenes(n: usize, allocator: Allocator) ![]bool {
    var is_prime = try allocator.alloc(bool, n + 1);
    for (is_prime) |*p| {
        p.* = true;
    }

    if (n >= 0) is_prime[0] = false;
    if (n >= 1) is_prime[1] = false;

    var i: usize = 2;
    while (i * i <= n) {
        if (is_prime[i]) {
            var j: usize = i * i;
            while (j <= n) {
                is_prime[j] = false;
                j += i;
            }
        }
        i += 1;
    }

    return is_prime;
}

pub fn factorial(n: i64) i64 {
    if (n <= 1) return 1;
    return n * factorial(n - 1);
}

pub fn permutation(n: i64, r: i64) i64 {
    if (r > n or r < 0) return 0;
    var result: i64 = 1;
    var i: i64 = 0;
    while (i < r) {
        result *= (n - i);
        i += 1;
    }
    return result;
}

pub fn combination(n: i64, r: i64) i64 {
    if (r > n or r < 0) return 0;
    var actual_r = r;
    if (r > n - r) {
        actual_r = n - r;
    }

    var result: i64 = 1;
    var i: i64 = 0;
    while (i < actual_r) {
        result = @divExact(result * (n - i), (i + 1));
        i += 1;
    }
    return result;
}

pub fn reverseString(s: []u8) void {
    var i: usize = 0;
    var j: usize = s.len - 1;
    while (i < j) {
        const temp = s[i];
        s[i] = s[j];
        s[j] = temp;
        i += 1;
        j -= 1;
    }
}

pub fn isPalindrome(s: []const u8) bool {
    var i: usize = 0;
    var j: usize = s.len - 1;
    while (i < j) {
        if (s[i] != s[j]) return false;
        i += 1;
        j -= 1;
    }
    return true;
}

pub fn sortSlice(comptime T: type, arr: []T) void {
    std.sort.sort(T, arr, {}, comptime std.sort.asc(T));
}

pub fn sortSliceDesc(comptime T: type, arr: []T) void {
    std.sort.sort(T, arr, {}, comptime std.sort.desc(T));
}

pub fn binarySearch(comptime T: type, arr: []const T, target: T) ?usize {
    var left: usize = 0;
    var right: usize = arr.len;

    while (left < right) {
        const mid = (left + right) / 2;
        if (arr[mid] == target) {
            return mid;
        } else if (arr[mid] < target) {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    return null;
}

pub fn printSlice(comptime T: type, arr: []const T) void {
    for (arr, 0..) |item, i| {
        if (i > 0) print(" ", .{});
        print("{}", .{item});
    }
    print("\n", .{});
}

pub fn printSliceLines(comptime T: type, arr: []const T) void {
    for (arr) |item| {
        print("{}\n", .{item});
    }
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const n = try readInt(i32);
    const arr = try readInts(i32, allocator);
    defer allocator.free(arr);

    print("Input: n={}, arr=", .{n});
    printSlice(i32, arr);

    const arr_i64 = try allocator.alloc(i64, arr.len);
    defer allocator.free(arr_i64);
    for (arr, 0..) |item, i| {
        arr_i64[i] = item;
    }

    print("Max: {}, Min: {}, Sum: {}\n", .{ maxSlice(i64, arr_i64), minSlice(i64, arr_i64), sumSlice(i64, arr_i64) });
    print("GCD of first two: {}\n", .{gcd(i64, arr_i64[0], arr_i64[1])});
    print("Is {} prime? {}\n", .{ n, isPrime(n) });

    print("Permutation of 5, 2: {d}\n", .{permutation(5, 2)});
    print("Combination of 5, 2: {d}\n", .{combination(5, 2)});

    print("\n--- read2DInts example ---\n", .{});
    print("Enter number of rows for 2D int array: ", .{});
    const n_2d = try readInt(usize);
    print("Enter {} lines of space-separated integers:\n", .{n_2d});
    const data_2d = try read2DInts(i32, allocator, n_2d);
    defer free2DSlice(i32, allocator, data_2d);

    print("2D Int Array:\n", .{});
    var i: usize = 0;
    while (i < data_2d.len) : (i += 1) {
        print("Row {}: ", .{i});
        printSlice(i32, data_2d[i]);
    }
}
