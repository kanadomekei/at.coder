const std = @import("std");

pub fn main() !void {
    // ==========================================
    // 配列とスライス
    // ==========================================

    std.debug.print("=== Zigの配列とスライス ===\n", .{});

    // ------------------------------------------
    // 基本的な配列
    // ------------------------------------------

    std.debug.print("\n=== 基本的な配列 ===\n", .{});

    // 固定長配列の宣言と初期化
    const numbers = [5]i32{ 1, 2, 3, 4, 5 };
    std.debug.print("固定長配列: ", .{});
    for (numbers) |num| {
        std.debug.print("{d} ", .{num});
    }
    std.debug.print("\n", .{});

    // 型推論を使った配列
    const auto_array = [_]i32{ 10, 20, 30, 40 };
    std.debug.print("型推論配列の長さ: {d}\n", .{auto_array.len});

    // 配列の要素アクセス
    std.debug.print("最初の要素: {d}, 最後の要素: {d}\n", .{ numbers[0], numbers[numbers.len - 1] });

    // ------------------------------------------
    // 配列の初期化方法
    // ------------------------------------------

    std.debug.print("\n=== 配列の初期化方法 ===\n", .{});

    // すべて同じ値で初期化
    const zeros = [_]i32{0} ** 5;
    std.debug.print("ゼロ配列: ", .{});
    for (zeros) |num| {
        std.debug.print("{d} ", .{num});
    }
    std.debug.print("\n", .{});

    // 未初期化配列（使用前に値を設定する必要がある）
    var uninitialized: [3]i32 = undefined;
    uninitialized[0] = 100;
    uninitialized[1] = 200;
    uninitialized[2] = 300;

    std.debug.print("後から初期化した配列: ", .{});
    for (uninitialized) |num| {
        std.debug.print("{d} ", .{num});
    }
    std.debug.print("\n", .{});

    // 部分的な初期化（残りは0で埋められる）
    const partial = [5]i32{ 1, 2, 0, 0, 0 };
    std.debug.print("部分初期化配列: ", .{});
    for (partial) |num| {
        std.debug.print("{d} ", .{num});
    }
    std.debug.print("\n", .{});

    // ------------------------------------------
    // 多次元配列
    // ------------------------------------------

    std.debug.print("\n=== 多次元配列 ===\n", .{});

    // 2次元配列
    const matrix = [3][3]i32{
        [_]i32{ 1, 2, 3 },
        [_]i32{ 4, 5, 6 },
        [_]i32{ 7, 8, 9 },
    };

    std.debug.print("2次元配列:\n", .{});
    for (matrix, 0..) |row, i| {
        std.debug.print("行 {d}: ", .{i});
        for (row) |value| {
            std.debug.print("{d} ", .{value});
        }
        std.debug.print("\n", .{});
    }

    // ------------------------------------------
    // スライスの基本
    // ------------------------------------------

    std.debug.print("\n=== スライスの基本 ===\n", .{});

    const source_array = [_]i32{ 10, 20, 30, 40, 50, 60, 70, 80, 90, 100 };

    // 基本的なスライス
    const slice1 = source_array[2..5]; // インデックス2から4まで
    std.debug.print("スライス [2..5]: ", .{});
    for (slice1) |value| {
        std.debug.print("{d} ", .{value});
    }
    std.debug.print("\n", .{});

    // 開始位置のみ指定
    const slice2 = source_array[7..source_array.len]; // インデックス7から最後まで
    std.debug.print("スライス [7..]: ", .{});
    for (slice2) |value| {
        std.debug.print("{d} ", .{value});
    }
    std.debug.print("\n", .{});

    // 終了位置のみ指定
    const slice3 = source_array[0..3]; // 最初からインデックス2まで
    std.debug.print("スライス [..3]: ", .{});
    for (slice3) |value| {
        std.debug.print("{d} ", .{value});
    }
    std.debug.print("\n", .{});

    // 全体をスライス
    const full_slice = source_array[0..source_array.len];
    std.debug.print("全体スライス長さ: {d}\n", .{full_slice.len});

    // ------------------------------------------
    // スライスの操作
    // ------------------------------------------

    std.debug.print("\n=== スライスの操作 ===\n", .{});

    var mutable_array = [_]i32{ 1, 2, 3, 4, 5, 6, 7, 8 };
    const mutable_slice = mutable_array[2..6];

    std.debug.print("変更前のスライス: ", .{});
    for (mutable_slice) |value| {
        std.debug.print("{d} ", .{value});
    }
    std.debug.print("\n", .{});

    // スライスの要素を変更
    for (mutable_slice) |*value| {
        value.* *= 10;
    }

    std.debug.print("変更後のスライス: ", .{});
    for (mutable_slice) |value| {
        std.debug.print("{d} ", .{value});
    }
    std.debug.print("\n", .{});

    std.debug.print("元の配列も変更されている: ", .{});
    for (mutable_array) |value| {
        std.debug.print("{d} ", .{value});
    }
    std.debug.print("\n", .{});

    // ------------------------------------------
    // 文字列配列とスライス
    // ------------------------------------------

    std.debug.print("\n=== 文字列配列とスライス ===\n", .{});

    const hello = "Hello, Zig!";
    std.debug.print("文字列の長さ: {d}\n", .{hello.len});

    // 文字列のスライス
    const greeting = hello[0..5]; // "Hello"
    const target = hello[7..10]; // "Zig"

    std.debug.print("挨拶部分: {s}\n", .{greeting});
    std.debug.print("対象部分: {s}\n", .{target});

    // UTF-8エンコーディングの処理
    const japanese = "こんにちは、Zig！";
    std.debug.print("日本語文字列のバイト長: {d}\n", .{japanese.len});

    // 文字列の各バイトを表示
    std.debug.print("バイト表現: ", .{});
    for (japanese[0..6]) |byte| { // "こん" の部分
        std.debug.print("{d} ", .{byte});
    }
    std.debug.print("\n", .{});

    // ------------------------------------------
    // 配列とスライスの比較・検索
    // ------------------------------------------

    std.debug.print("\n=== 配列とスライスの比較・検索 ===\n", .{});

    const search_array = [_]i32{ 10, 25, 30, 45, 50, 65, 70, 85, 90 };

    // 要素の検索
    const target_value = 45;
    const found_index = find_element(search_array[0..search_array.len], target_value);

    if (found_index) |index| {
        std.debug.print("値 {d} がインデックス {d} で見つかりました\n", .{ target_value, index });
    } else {
        std.debug.print("値 {d} は見つかりませんでした\n", .{target_value});
    }

    // 配列の最大値・最小値
    const max_value = find_max_value(search_array[0..search_array.len]);
    const min_value = find_min_value(search_array[0..search_array.len]);
    std.debug.print("最大値: {d}, 最小値: {d}\n", .{ max_value, min_value });

    // ------------------------------------------
    // 配列のソート
    // ------------------------------------------

    std.debug.print("\n=== 配列のソート ===\n", .{});

    var unsorted = [_]i32{ 64, 34, 25, 12, 22, 11, 90 };

    std.debug.print("ソート前: ", .{});
    for (unsorted) |value| {
        std.debug.print("{d} ", .{value});
    }
    std.debug.print("\n", .{});

    // バブルソート
    bubble_sort(unsorted[0..unsorted.len]);

    std.debug.print("ソート後: ", .{});
    for (unsorted) |value| {
        std.debug.print("{d} ", .{value});
    }
    std.debug.print("\n", .{});

    // ------------------------------------------
    // 配列の連結とコピー
    // ------------------------------------------

    std.debug.print("\n=== 配列の連結とコピー ===\n", .{});

    const array1 = [_]i32{ 1, 2, 3 };
    const array2 = [_]i32{ 4, 5, 6 };

    // 配列の連結（コンパイル時）
    const concatenated = array1 ++ array2;
    std.debug.print("連結された配列: ", .{});
    for (concatenated) |value| {
        std.debug.print("{d} ", .{value});
    }
    std.debug.print("\n", .{});

    // 配列のコピー
    var copied_array: [6]i32 = undefined;
    @memcpy(copied_array[0..3], &array1);
    @memcpy(copied_array[3..6], &array2);

    std.debug.print("コピーされた配列: ", .{});
    for (copied_array) |value| {
        std.debug.print("{d} ", .{value});
    }
    std.debug.print("\n", .{});

    // ------------------------------------------
    // センチネル終端配列
    // ------------------------------------------

    std.debug.print("\n=== センチネル終端配列 ===\n", .{});

    // null終端文字列（C文字列互換）
    const c_string: [*:0]const u8 = "Hello from C!";
    std.debug.print("C互換文字列: {s}\n", .{c_string});

    // センチネル値付き配列
    const sentinel_array = [_:0]i32{ 1, 2, 3, 4, 5 };
    std.debug.print("センチネル配列の長さ: {d}\n", .{sentinel_array.len});
    std.debug.print("センチネル値: {d}\n", .{sentinel_array[sentinel_array.len]});
}

// ------------------------------------------
// ユーティリティ関数
// ------------------------------------------

// 要素の検索
fn find_element(slice: []const i32, target: i32) ?usize {
    for (slice, 0..) |value, index| {
        if (value == target) {
            return index;
        }
    }
    return null;
}

// 最大値を見つける
fn find_max_value(slice: []const i32) i32 {
    var max = slice[0];
    for (slice[1..]) |value| {
        if (value > max) {
            max = value;
        }
    }
    return max;
}

// 最小値を見つける
fn find_min_value(slice: []const i32) i32 {
    var min = slice[0];
    for (slice[1..]) |value| {
        if (value < min) {
            min = value;
        }
    }
    return min;
}

// バブルソート
fn bubble_sort(slice: []i32) void {
    const n = slice.len;
    var i: usize = 0;
    while (i < n) : (i += 1) {
        var j: usize = 0;
        while (j < n - 1 - i) : (j += 1) {
            if (slice[j] > slice[j + 1]) {
                // 要素の交換
                const temp = slice[j];
                slice[j] = slice[j + 1];
                slice[j + 1] = temp;
            }
        }
    }
}

// 配列の合計を計算
fn sum_array(slice: []const i32) i32 {
    var total: i32 = 0;
    for (slice) |value| {
        total += value;
    }
    return total;
}

// 配列の平均を計算
fn average_array(slice: []const i32) f32 {
    if (slice.len == 0) return 0.0;
    const total = sum_array(slice);
    return @as(f32, @floatFromInt(total)) / @as(f32, @floatFromInt(slice.len));
}
