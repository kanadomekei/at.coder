const std = @import("std");

pub fn main() !void {
    // ==========================================
    // コンパイル時計算（comptime）
    // ==========================================

    std.debug.print("=== Zigのコンパイル時計算 ===\n", .{});

    // ------------------------------------------
    // 基本的なcomptime
    // ------------------------------------------

    std.debug.print("\n=== 基本的なcomptime ===\n", .{});

    // コンパイル時に計算される定数
    const compile_time_result = comptime fibonacci(10);
    std.debug.print("コンパイル時に計算されたフィボナッチ数列の10番目: {d}\n", .{compile_time_result});

    // コンパイル時の型生成
    const IntArray = comptime create_array_type(i32, 5);
    const int_array: IntArray = [_]i32{ 1, 2, 3, 4, 5 };

    std.debug.print("コンパイル時に生成された型の配列: ", .{});
    for (int_array) |value| {
        std.debug.print("{d} ", .{value});
    }
    std.debug.print("\n", .{});

    // ------------------------------------------
    // comptime引数
    // ------------------------------------------

    std.debug.print("\n=== comptime引数 ===\n", .{});

    // ジェネリック関数でのcomptime使用
    const result1 = generic_add(i32, 10, 20);
    const result2 = generic_add(f32, 3.14, 2.86);

    std.debug.print("ジェネリック加算 (i32): {d}\n", .{result1});
    std.debug.print("ジェネリック加算 (f32): {d}\n", .{result2});

    // 配列サイズをcomptime引数として受け取る
    const small_array = create_initialized_array(3, 42);
    const large_array = create_initialized_array(7, 100);

    std.debug.print("小さな配列: ", .{});
    for (small_array) |value| {
        std.debug.print("{d} ", .{value});
    }
    std.debug.print("\n", .{});

    std.debug.print("大きな配列: ", .{});
    for (large_array) |value| {
        std.debug.print("{d} ", .{value});
    }
    std.debug.print("\n", .{});

    // ------------------------------------------
    // コンパイル時の型操作
    // ------------------------------------------

    std.debug.print("\n=== コンパイル時の型操作 ===\n", .{});

    // 型情報の取得
    print_type_info(@TypeOf(42));
    print_type_info(@TypeOf(3.14));
    print_type_info(@TypeOf("Hello"));
    print_type_info(@TypeOf(true));

    // 構造体の型情報
    const Person = struct {
        name: []const u8,
        age: u32,
        height: f32,
    };

    print_struct_info(Person);

    // ------------------------------------------
    // コンパイル時のループ
    // ------------------------------------------

    std.debug.print("\n=== コンパイル時のループ ===\n", .{});

    // コンパイル時に複数の型で同じ処理を実行
    const types = [_]type{ i8, i16, i32, i64, u8, u16, u32, u64 };

    std.debug.print("各整数型のサイズ:\n", .{});
    inline for (types) |T| {
        std.debug.print("{s}: {d} bytes\n", .{ @typeName(T), @sizeOf(T) });
    }

    // ------------------------------------------
    // コンパイル時の条件分岐
    // ------------------------------------------

    std.debug.print("\n=== コンパイル時の条件分岐 ===\n", .{});

    // プラットフォーム固有のコード
    const platform_specific = comptime get_platform_info();
    std.debug.print("プラットフォーム情報: {s}\n", .{platform_specific});

    // デバッグビルド専用のコード
    if (comptime std.debug.runtime_safety) {
        std.debug.print("デバッグビルドで実行中\n", .{});
    } else {
        std.debug.print("リリースビルドで実行中\n", .{});
    }

    // ------------------------------------------
    // メタプログラミング
    // ------------------------------------------

    std.debug.print("\n=== メタプログラミング ===\n", .{});

    // 構造体の自動生成
    const Point2D = create_point_type(f32, 2);
    const Point3D = create_point_type(f64, 3);

    const p2d = Point2D{ .x = 1.0, .y = 2.0 };
    const p3d = Point3D{ .x = 1.0, .y = 2.0, .z = 3.0 };

    std.debug.print("2D点: ({d}, {d})\n", .{ p2d.x, p2d.y });
    std.debug.print("3D点: ({d}, {d}, {d})\n", .{ p3d.x, p3d.y, p3d.z });

    // ------------------------------------------
    // コンパイル時の文字列操作
    // ------------------------------------------

    std.debug.print("\n=== コンパイル時の文字列操作 ===\n", .{});

    // コンパイル時に文字列を結合
    const greeting = comptime "Hello, " ++ "Zig!";
    std.debug.print("コンパイル時結合文字列: {s}\n", .{greeting});

    // 型名を使った動的な文字列生成
    const type_message = comptime "This is a " ++ @typeName(i32);
    std.debug.print("型名メッセージ: {s}\n", .{type_message});

    // ------------------------------------------
    // ビルド時設定
    // ------------------------------------------

    std.debug.print("\n=== ビルド時設定 ===\n", .{});

    // ビルド時の設定に基づく条件分岐
    const config = comptime BuildConfig{
        .debug_mode = true,
        .optimization_level = 2,
        .feature_flags = .{ .networking = true, .graphics = false },
    };

    if (comptime config.debug_mode) {
        std.debug.print("デバッグモードが有効です\n", .{});
    }

    std.debug.print("最適化レベル: {d}\n", .{config.optimization_level});

    if (comptime config.feature_flags.networking) {
        std.debug.print("ネットワーク機能が有効です\n", .{});
    }

    if (comptime config.feature_flags.graphics) {
        std.debug.print("グラフィック機能が有効です\n", .{});
    } else {
        std.debug.print("グラフィック機能は無効です\n", .{});
    }

    // ------------------------------------------
    // コンパイル時テスト
    // ------------------------------------------

    std.debug.print("\n=== コンパイル時テスト ===\n", .{});

    // コンパイル時にテストを実行
    comptime {
        const test_result = fibonacci(5);
        if (test_result != 5) {
            @compileError("fibonacci(5) should return 5");
        }
    }

    std.debug.print("コンパイル時テストが成功しました\n", .{});
}

// ------------------------------------------
// コンパイル時関数
// ------------------------------------------

// フィボナッチ数列（コンパイル時計算可能）
fn fibonacci(n: u32) u32 {
    if (n <= 1) return n;
    return fibonacci(n - 1) + fibonacci(n - 2);
}

// 配列型を動的に生成
fn create_array_type(comptime T: type, comptime size: usize) type {
    return [size]T;
}

// ジェネリック関数
fn generic_add(comptime T: type, a: T, b: T) T {
    return a + b;
}

// コンパイル時に配列を初期化
fn create_initialized_array(comptime size: usize, value: i32) [size]i32 {
    return [_]i32{value} ** size;
}

// 型情報を表示
fn print_type_info(comptime T: type) void {
    const info = @typeInfo(T);

    std.debug.print("型: {s}, 種類: ", .{@typeName(T)});

    switch (info) {
        .int => |int_info| {
            const sign = if (int_info.signedness == .signed) "signed" else "unsigned";
            std.debug.print("整数 ({s}, {d}bit)\n", .{ sign, int_info.bits });
        },
        .float => |float_info| {
            std.debug.print("浮動小数点 ({d}bit)\n", .{float_info.bits});
        },
        .bool => std.debug.print("論理型\n", .{}),
        .pointer => |ptr_info| {
            std.debug.print("ポインタ (子の型: {s})\n", .{@typeName(ptr_info.child)});
        },
        else => std.debug.print("その他\n", .{}),
    }
}

// 構造体の情報を表示
fn print_struct_info(comptime T: type) void {
    const info = @typeInfo(T);

    switch (info) {
        .@"struct" => {},
        else => {
            std.debug.print("{s} は構造体ではありません\n", .{@typeName(T)});
            return;
        },
    }

    const struct_info = info.@"struct";
    std.debug.print("構造体 {s} のフィールド数: {d}\n", .{ @typeName(T), struct_info.fields.len });

    inline for (struct_info.fields) |field| {
        std.debug.print("- {s}: {s}\n", .{ field.name, @typeName(field.type) });
    }
}

// プラットフォーム情報を取得
fn get_platform_info() []const u8 {
    return switch (@import("builtin").target.os.tag) {
        .windows => "Windows",
        .macos => "macOS",
        .linux => "Linux",
        .freebsd => "FreeBSD",
        else => "Unknown",
    };
}

// 動的な点の型を生成
fn create_point_type(comptime T: type, comptime dimensions: u32) type {
    return switch (dimensions) {
        2 => struct {
            x: T,
            y: T,
        },
        3 => struct {
            x: T,
            y: T,
            z: T,
        },
        else => @compileError("Unsupported number of dimensions"),
    };
}

// ------------------------------------------
// ビルド時設定構造体
// ------------------------------------------

const BuildConfig = struct {
    debug_mode: bool,
    optimization_level: u8,
    feature_flags: FeatureFlags,

    const FeatureFlags = struct {
        networking: bool,
        graphics: bool,
    };
};

// ------------------------------------------
// テスト
// ------------------------------------------

test "コンパイル時計算のテスト" {
    // コンパイル時に計算される
    const result = comptime fibonacci(7);
    try std.testing.expect(result == 13);
}

test "型生成のテスト" {
    const ArrayType = comptime create_array_type(u32, 3);
    const array: ArrayType = [_]u32{ 1, 2, 3 };
    try std.testing.expect(array.len == 3);
}

test "メタプログラミングのテスト" {
    const Point = create_point_type(i32, 2);
    const p = Point{ .x = 10, .y = 20 };
    try std.testing.expect(p.x == 10);
    try std.testing.expect(p.y == 20);
}
