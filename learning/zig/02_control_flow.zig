const std = @import("std");

pub fn main() !void {
    // ==========================================
    // 制御構文
    // ==========================================

    std.debug.print("=== Zigの制御構文 ===\n", .{});

    // ------------------------------------------
    // if文
    // ------------------------------------------

    std.debug.print("\n=== if文 ===\n", .{});

    const score = 85;

    // 基本的なif文
    if (score >= 90) {
        std.debug.print("優秀です！\n", .{});
    } else if (score >= 70) {
        std.debug.print("良い成績です。\n", .{});
    } else if (score >= 50) {
        std.debug.print("合格です。\n", .{});
    } else {
        std.debug.print("がんばりましょう。\n", .{});
    }

    // if式（値を返す）
    const grade = if (score >= 90) "A" else if (score >= 80) "B" else if (score >= 70) "C" else "D";
    std.debug.print("成績: {s}\n", .{grade});

    // Optional型とif文の組み合わせ
    const maybe_value: ?i32 = 42;
    if (maybe_value) |value| {
        std.debug.print("値が存在します: {d}\n", .{value});
    } else {
        std.debug.print("値がありません\n", .{});
    }

    // ------------------------------------------
    // while文
    // ------------------------------------------

    std.debug.print("\n=== while文 ===\n", .{});

    // 基本的なwhile文
    var i: u32 = 0;
    std.debug.print("カウントアップ: ", .{});
    while (i < 5) {
        std.debug.print("{d} ", .{i});
        i += 1;
    }
    std.debug.print("\n", .{});

    // while文でOptional型を扱う
    const numbers = [_]i32{ 1, 2, 3, 4, 5 };
    var index: usize = 0;

    std.debug.print("配列の要素: ", .{});
    while (index < numbers.len) : (index += 1) {
        std.debug.print("{d} ", .{numbers[index]});
    }
    std.debug.print("\n", .{});

    // continueラベル付きwhile
    var counter: u32 = 0;
    std.debug.print("偶数のみ: ", .{});
    while (counter < 10) : (counter += 1) {
        if (counter % 2 != 0) continue;
        std.debug.print("{d} ", .{counter});
    }
    std.debug.print("\n", .{});

    // ------------------------------------------
    // for文
    // ------------------------------------------

    std.debug.print("\n=== for文 ===\n", .{});

    // 配列に対するfor文
    const fruits = [_][]const u8{ "apple", "banana", "cherry", "date" };

    std.debug.print("果物リスト:\n", .{});
    for (fruits) |fruit| {
        std.debug.print("- {s}\n", .{fruit});
    }

    // インデックス付きfor文
    std.debug.print("インデックス付き:\n", .{});
    for (fruits, 0..) |fruit, idx| {
        std.debug.print("{d}: {s}\n", .{ idx, fruit });
    }

    // 範囲for文（スライス）
    const slice = fruits[1..3];
    std.debug.print("スライス要素: ", .{});
    for (slice) |item| {
        std.debug.print("{s} ", .{item});
    }
    std.debug.print("\n", .{});

    // for文による配列の変更
    var mutable_numbers = [_]i32{ 1, 2, 3, 4, 5 };
    std.debug.print("変更前: ", .{});
    for (mutable_numbers) |num| {
        std.debug.print("{d} ", .{num});
    }
    std.debug.print("\n", .{});

    // ポインタを使って配列の要素を変更
    for (&mutable_numbers) |*num| {
        num.* *= 2;
    }

    std.debug.print("変更後: ", .{});
    for (mutable_numbers) |num| {
        std.debug.print("{d} ", .{num});
    }
    std.debug.print("\n", .{});

    // ------------------------------------------
    // switch文
    // ------------------------------------------

    std.debug.print("\n=== switch文 ===\n", .{});

    const day = 3;

    // 基本的なswitch文
    switch (day) {
        1 => std.debug.print("月曜日\n", .{}),
        2 => std.debug.print("火曜日\n", .{}),
        3 => std.debug.print("水曜日\n", .{}),
        4 => std.debug.print("木曜日\n", .{}),
        5 => std.debug.print("金曜日\n", .{}),
        6, 7 => std.debug.print("週末\n", .{}), // 複数の値をまとめて処理
        else => std.debug.print("無効な日付\n", .{}),
    }

    // switch式（値を返す）
    const day_type = switch (day) {
        1...5 => "平日",
        6, 7 => "週末",
        else => "無効",
    };
    std.debug.print("日付タイプ: {s}\n", .{day_type});

    // 文字に対するswitch
    const letter = 'A';
    switch (letter) {
        'A', 'E', 'I', 'O', 'U' => std.debug.print("'{c}' は母音です\n", .{letter}),
        'a', 'e', 'i', 'o', 'u' => std.debug.print("'{c}' は母音です（小文字）\n", .{letter}),
        else => std.debug.print("'{c}' は子音です\n", .{letter}),
    }

    // ------------------------------------------
    // ネストしたループとラベル
    // ------------------------------------------

    std.debug.print("\n=== ネストしたループとラベル ===\n", .{});

    // ラベル付きループ
    outer: for (0..3) |i_outer| {
        for (0..3) |j_inner| {
            if (i_outer == 1 and j_inner == 1) {
                std.debug.print("外側のループを終了\n", .{});
                break :outer;
            }
            std.debug.print("({d}, {d}) ", .{ i_outer, j_inner });
        }
    }
    std.debug.print("\n", .{});

    // continueラベル
    std.debug.print("continueラベルの例:\n", .{});
    outer_continue: for (0..3) |i_outer| {
        for (0..3) |j_inner| {
            if (j_inner == 1) {
                std.debug.print("外側のループの次の反復へ\n", .{});
                continue :outer_continue;
            }
            std.debug.print("({d}, {d}) ", .{ i_outer, j_inner });
        }
        std.debug.print("\n", .{});
    }

    // ------------------------------------------
    // defer文
    // ------------------------------------------

    std.debug.print("\n=== defer文 ===\n", .{});

    {
        std.debug.print("処理開始\n", .{});
        defer std.debug.print("処理終了（defer）\n", .{});
        defer std.debug.print("cleanup処理（defer）\n", .{});

        std.debug.print("メイン処理\n", .{});
        // スコープを抜ける際に、defer文が逆順で実行される
    }

    // deferは関数レベルでも使用可能
    defer_example();
}

fn defer_example() void {
    std.debug.print("関数開始\n", .{});
    defer std.debug.print("関数終了（defer）\n", .{});

    var i: u32 = 0;
    while (i < 3) {
        defer std.debug.print("ループ終了 {d}（defer）\n", .{i});
        std.debug.print("ループ {d}\n", .{i});
        i += 1;
    }
}
