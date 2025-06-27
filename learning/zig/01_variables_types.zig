const std = @import("std");

pub fn main() !void {
    // ==========================================
    // 変数とデータ型
    // ==========================================

    std.debug.print("=== Zigの変数とデータ型 ===\n", .{});

    // ------------------------------------------
    // 変数の宣言
    // ------------------------------------------

    // const: 定数（変更不可）
    const pi: f32 = 3.14159;
    std.debug.print("定数 pi: {d}\n", .{pi});

    // var: 変数（変更可能）
    var counter: i32 = 0;
    std.debug.print("初期値 counter: {d}\n", .{counter});
    counter += 10;
    std.debug.print("変更後 counter: {d}\n", .{counter});

    // 型推論を使った宣言
    const auto_int = 42; // i32 として推論
    const auto_float = 3.14; // f64 として推論
    const auto_string = "Hello"; // *const [5:0]u8 として推論

    std.debug.print("型推論: int={d}, float={d}, string={s}\n", .{ auto_int, auto_float, auto_string });

    // ------------------------------------------
    // 整数型
    // ------------------------------------------

    std.debug.print("\n=== 整数型 ===\n", .{});

    // 符号付き整数
    const i8_val: i8 = -128; // 8bit: -128 to 127
    const i16_val: i16 = -32768; // 16bit: -32768 to 32767
    const i32_val: i32 = -2147483648; // 32bit
    const i64_val: i64 = -9223372036854775808; // 64bit

    // 符号なし整数
    const u8_val: u8 = 255; // 8bit: 0 to 255
    const u16_val: u16 = 65535; // 16bit: 0 to 65535
    const u32_val: u32 = 4294967295; // 32bit
    const u64_val: u64 = 18446744073709551615; // 64bit

    std.debug.print("i8: {d}, u8: {d}\n", .{ i8_val, u8_val });
    std.debug.print("i16: {d}, u16: {d}\n", .{ i16_val, u16_val });
    std.debug.print("i32: {d}, u32: {d}\n", .{ i32_val, u32_val });
    std.debug.print("i64: {d}, u64: {d}\n", .{ i64_val, u64_val });

    // カスタムビット幅整数
    const i3_val: i3 = -4; // 3bit符号付き: -4 to 3
    const u3_val: u3 = 7; // 3bit符号なし: 0 to 7
    std.debug.print("カスタムビット幅 i3: {d}, u3: {d}\n", .{ i3_val, u3_val });

    // ------------------------------------------
    // 浮動小数点型
    // ------------------------------------------

    std.debug.print("\n=== 浮動小数点型 ===\n", .{});

    const f16_val: f16 = 1.5; // 16bit浮動小数点
    const f32_val: f32 = 3.14159; // 32bit浮動小数点
    const f64_val: f64 = 2.71828182845904523536; // 64bit浮動小数点
    const f128_val: f128 = 1.41421356237309504880168872420969807856967187537694; // 128bit浮動小数点

    std.debug.print("f16: {d}\n", .{f16_val});
    std.debug.print("f32: {d}\n", .{f32_val});
    std.debug.print("f64: {d}\n", .{f64_val});
    std.debug.print("f128: {d}\n", .{f128_val});

    // ------------------------------------------
    // 論理型
    // ------------------------------------------

    std.debug.print("\n=== 論理型 ===\n", .{});

    const is_true: bool = true;
    const is_false: bool = false;

    std.debug.print("true: {}, false: {}\n", .{ is_true, is_false });

    // 論理演算
    const and_result = is_true and is_false; // false
    const or_result = is_true or is_false; // true
    const not_result = !is_true; // false

    std.debug.print("and: {}, or: {}, not: {}\n", .{ and_result, or_result, not_result });

    // ------------------------------------------
    // 文字・文字列型
    // ------------------------------------------

    std.debug.print("\n=== 文字・文字列型 ===\n", .{});

    // 文字（UTF-8コードポイント）
    const char_a: u8 = 'A';
    const char_unicode: u21 = '🦎'; // Zigのマスコット

    std.debug.print("文字 A: {c} (コード: {d})\n", .{ char_a, char_a });
    std.debug.print("Unicode文字: {u} (コード: {d})\n", .{ char_unicode, char_unicode });

    // 文字列リテラル
    const greeting = "Hello, Zig!";
    const multiline =
        \\これは複数行の
        \\文字列です。
        \\各行の先頭の\\は削除されます。
    ;

    std.debug.print("挨拶: {s}\n", .{greeting});
    std.debug.print("複数行:\n{s}\n", .{multiline});

    // ------------------------------------------
    // Optional型（nullable）
    // ------------------------------------------

    std.debug.print("\n=== Optional型 ===\n", .{});

    // Optional型は ? を使って宣言
    const maybe_number: ?i32 = 42;
    const empty_number: ?i32 = null;

    // Optional型のチェック
    if (maybe_number) |value| {
        std.debug.print("値があります: {d}\n", .{value});
    } else {
        std.debug.print("値がありません\n", .{});
    }

    if (empty_number) |value| {
        std.debug.print("値があります: {d}\n", .{value});
    } else {
        std.debug.print("値がありません（null）\n", .{});
    }

    // ------------------------------------------
    // 配列とスライス（基本）
    // ------------------------------------------

    std.debug.print("\n=== 配列とスライス（基本） ===\n", .{});

    // 固定長配列
    const numbers = [_]i32{ 1, 2, 3, 4, 5 };
    std.debug.print("配列の長さ: {d}\n", .{numbers.len});
    std.debug.print("最初の要素: {d}\n", .{numbers[0]});

    // スライス
    const slice = numbers[1..4]; // インデックス1から3まで
    std.debug.print("スライス: ", .{});
    for (slice) |value| {
        std.debug.print("{d} ", .{value});
    }
    std.debug.print("\n", .{});

    // ------------------------------------------
    // 型キャスト
    // ------------------------------------------

    std.debug.print("\n=== 型キャスト ===\n", .{});

    const small_int: i8 = 100;
    const big_int: i32 = small_int; // 暗黙的な拡張キャスト

    const big_value: i32 = 127;
    const small_value: i8 = @intCast(big_value); // 明示的な縮小キャスト（範囲チェックあり）

    std.debug.print("i8 -> i32: {d} -> {d}\n", .{ small_int, big_int });
    std.debug.print("i32 -> i8: {d} -> {d}\n", .{ big_value, small_value });

    // floatとintの変換
    const float_val: f32 = 3.7;
    const int_from_float: i32 = @intFromFloat(float_val);
    const float_from_int: f32 = @floatFromInt(int_from_float);

    std.debug.print("float -> int -> float: {d} -> {d} -> {d}\n", .{ float_val, int_from_float, float_from_int });
}
