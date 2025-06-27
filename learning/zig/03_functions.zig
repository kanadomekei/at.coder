const std = @import("std");

pub fn main() !void {
    // ==========================================
    // 関数
    // ==========================================

    std.debug.print("=== Zigの関数 ===\n", .{});

    // ------------------------------------------
    // 基本的な関数呼び出し
    // ------------------------------------------

    std.debug.print("\n=== 基本的な関数 ===\n", .{});

    // 引数なし、返り値なしの関数
    greet();

    // 引数あり、返り値なしの関数
    greet_person("Alice");

    // 引数あり、返り値ありの関数
    const sum = add(10, 20);
    std.debug.print("10 + 20 = {d}\n", .{sum});

    // 複数の引数と返り値
    const result = calculate(15, 3);
    std.debug.print("15と3の計算結果: 和={d}, 差={d}, 積={d}\n", .{ result.sum, result.diff, result.product });

    // ------------------------------------------
    // 引数の種類
    // ------------------------------------------

    std.debug.print("\n=== 引数の種類 ===\n", .{});

    // 値渡し
    var number: i32 = 42;
    modify_value(number);
    std.debug.print("値渡し後: {d}\n", .{number}); // 変更されない

    // 参照渡し（ポインタ）
    modify_reference(&number);
    std.debug.print("参照渡し後: {d}\n", .{number}); // 変更される

    // 配列の渡し方
    const numbers = [_]i32{ 1, 2, 3, 4, 5 };
    print_array(&numbers);

    // スライスの渡し方
    const slice = numbers[1..4];
    print_slice(slice);

    // ------------------------------------------
    // 可変長引数
    // ------------------------------------------

    std.debug.print("\n=== 可変長引数 ===\n", .{});

    print_numbers(.{ 1, 2, 3 });
    print_numbers(.{ 10, 20, 30, 40, 50 });

    // ------------------------------------------
    // 関数ポインタ
    // ------------------------------------------

    std.debug.print("\n=== 関数ポインタ ===\n", .{});

    // 関数ポインタの宣言と使用
    const add_func = add;
    const result_from_ptr = add_func(5, 7);
    std.debug.print("関数ポインタ経由: {d}\n", .{result_from_ptr});

    // 関数を引数として渡す
    const add_result = apply_operation(10, 5, add);
    const mult_result = apply_operation(10, 5, multiply);
    std.debug.print("apply_operation(10, 5, add): {d}\n", .{add_result});
    std.debug.print("apply_operation(10, 5, multiply): {d}\n", .{mult_result});

    // ------------------------------------------
    // 匿名関数（クロージャ）
    // ------------------------------------------

    std.debug.print("\n=== 匿名関数 ===\n", .{});

    // 匿名関数の定義と使用
    const square = struct {
        fn call(x: i32) i32 {
            return x * x;
        }
    }.call;

    std.debug.print("5の二乗: {d}\n", .{square(5)});

    // ------------------------------------------
    // ジェネリック関数
    // ------------------------------------------

    std.debug.print("\n=== ジェネリック関数 ===\n", .{});

    // 型パラメータを持つ関数
    const max_int = max(i32, 10, 20);
    const max_float = max(f32, 3.14, 2.71);
    std.debug.print("max(10, 20): {d}\n", .{max_int});
    std.debug.print("max(3.14, 2.71): {d}\n", .{max_float});

    // ジェネリック関数でスライスを処理
    const int_slice = [_]i32{ 1, 5, 3, 9, 2 };
    const float_slice = [_]f32{ 1.1, 5.5, 3.3, 9.9, 2.2 };

    std.debug.print("整数配列の最大値: {d}\n", .{find_max(i32, &int_slice)});
    std.debug.print("浮動小数点配列の最大値: {d}\n", .{find_max(f32, &float_slice)});

    // ------------------------------------------
    // 再帰関数
    // ------------------------------------------

    std.debug.print("\n=== 再帰関数 ===\n", .{});

    const factorial_5 = factorial(5);
    std.debug.print("5! = {d}\n", .{factorial_5});

    const fib_10 = fibonacci(10);
    std.debug.print("fibonacci(10) = {d}\n", .{fib_10});

    // ------------------------------------------
    // エラーを返す関数
    // ------------------------------------------

    std.debug.print("\n=== エラーを返す関数 ===\n", .{});

    // 正常なケース
    if (divide(10, 2)) |div_result| {
        std.debug.print("10 / 2 = {d}\n", .{div_result});
    } else |err| {
        std.debug.print("エラー: {}\n", .{err});
    }

    // エラーケース
    if (divide(10, 0)) |div_result| {
        std.debug.print("10 / 0 = {d}\n", .{div_result});
    } else |err| {
        std.debug.print("エラー: {}\n", .{err});
    }
}

// ------------------------------------------
// 関数定義
// ------------------------------------------

// 引数なし、返り値なし
fn greet() void {
    std.debug.print("こんにちは！\n", .{});
}

// 引数あり、返り値なし
fn greet_person(name: []const u8) void {
    std.debug.print("こんにちは、{s}さん！\n", .{name});
}

// 引数あり、返り値あり
fn add(a: i32, b: i32) i32 {
    return a + b;
}

fn multiply(a: i32, b: i32) i32 {
    return a * b;
}

// 構造体を返す関数
const CalculationResult = struct {
    sum: i32,
    diff: i32,
    product: i32,
};

fn calculate(a: i32, b: i32) CalculationResult {
    return CalculationResult{
        .sum = a + b,
        .diff = a - b,
        .product = a * b,
    };
}

// 値渡しの例
fn modify_value(value: i32) void {
    _ = value + 100; // valueは変更されない（ローカルコピー）
}

// 参照渡しの例
fn modify_reference(value: *i32) void {
    value.* += 100; // 元の値が変更される
}

// 配列を受け取る関数
fn print_array(arr: *const [5]i32) void {
    std.debug.print("配列: ", .{});
    for (arr) |value| {
        std.debug.print("{d} ", .{value});
    }
    std.debug.print("\n", .{});
}

// スライスを受け取る関数
fn print_slice(slice: []const i32) void {
    std.debug.print("スライス: ", .{});
    for (slice) |value| {
        std.debug.print("{d} ", .{value});
    }
    std.debug.print("\n", .{});
}

// 可変長引数関数
fn print_numbers(args: anytype) void {
    std.debug.print("数値: ", .{});
    inline for (args) |arg| {
        std.debug.print("{d} ", .{arg});
    }
    std.debug.print("\n", .{});
}

// 関数を引数として受け取る関数
fn apply_operation(a: i32, b: i32, operation: fn (i32, i32) i32) i32 {
    return operation(a, b);
}

// ジェネリック関数
fn max(comptime T: type, a: T, b: T) T {
    return if (a > b) a else b;
}

// ジェネリック関数でスライスを処理
fn find_max(comptime T: type, slice: []const T) T {
    var maximum = slice[0];
    for (slice[1..]) |value| {
        if (value > maximum) {
            maximum = value;
        }
    }
    return maximum;
}

// 再帰関数：階乗
fn factorial(n: u32) u32 {
    if (n <= 1) return 1;
    return n * factorial(n - 1);
}

// 再帰関数：フィボナッチ数列
fn fibonacci(n: u32) u32 {
    if (n <= 1) return n;
    return fibonacci(n - 1) + fibonacci(n - 2);
}

// エラーを返す関数
const DivisionError = error{
    DivisionByZero,
};

fn divide(a: i32, b: i32) DivisionError!i32 {
    if (b == 0) return DivisionError.DivisionByZero;
    return @divTrunc(a, b);
}
