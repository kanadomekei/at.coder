# Zig 包括的文法ガイド

このディレクトリには、Zigプログラミング言語の包括的な文法と機能を学ぶためのサンプルコードが含まれています。

## 目次

1. [Zigとは](#zigとは)
2. [学習の進め方](#学習の進め方)
3. [ファイルの実行方法](#ファイルの実行方法)
4. [基本的なプログラム構造](#基本的なプログラム構造)
5. [変数とデータ型](#変数とデータ型)
6. [制御フロー](#制御フロー)
7. [関数](#関数)
8. [構造体とEnum](#構造体とenum)
9. [エラーハンドリング](#エラーハンドリング)
10. [メモリ管理](#メモリ管理)
11. [配列とスライス](#配列とスライス)
12. [コンパイル時機能](#コンパイル時機能)
13. [高度な機能](#高度な機能)

---

## Zigとは

Zigは、C言語の代替を目指して設計されたシステムプログラミング言語です。主な特徴：
- **メモリ安全性とパフォーマンスを両立**
- **明示的なエラーハンドリング**
- **コンパイル時計算の強力なサポート**
- **Cとの相互運用性**
- **隠れた制御フローなし**（例外処理やガベージコレクションなし）

## 学習の進め方

以下の順序でサンプルファイルを実行・確認することをお勧めします：

1. **01_variables_types.zig** - 変数とデータ型
2. **02_control_flow.zig** - 制御構文（if、while、for）
3. **03_functions.zig** - 関数の定義と呼び出し
4. **04_structs_enums.zig** - 構造体とenum
5. **05_error_handling.zig** - エラーハンドリング
6. **06_memory_management.zig** - メモリ管理
7. **07_arrays_slices.zig** - 配列とスライス
8. **08_comptime.zig** - コンパイル時計算
9. **09_practical_examples.zig** - 実践的なサンプル

## ファイルの実行方法

各ファイルは以下のコマンドで実行できます：

```bash
# 実行
zig run filename.zig

# コンパイル
zig build-exe filename.zig

# テスト実行（テストがある場合）
zig test filename.zig
```

## 基本的なプログラム構造

Zigプログラムの基本構造：

```zig
const std = @import("std");

pub fn main() !void {
    // メイン処理
    std.debug.print("Hello, Zig!\n", .{});
}
```

**重要なポイント:**
- `const std = @import("std");` で標準ライブラリをインポート
- `pub fn main()` が実行エントリーポイント
- `!void` はエラーを返す可能性があることを示す
- `.{}` は空のタプル（引数なし）

---

## 変数とデータ型

### 変数の宣言

```zig
// const: 定数（変更不可）
const pi: f32 = 3.14159;

// var: 変数（変更可能）
var counter: i32 = 0;
counter += 10; // 変更可能

// 型推論
const auto_int = 42;        // i32として推論
const auto_float = 3.14;    // f64として推論
const auto_string = "Hello"; // *const [5:0]u8として推論
```

**ポイント:**
- `const` は不変、`var` は可変
- 型は明示的に指定するか、推論させることができる
- 型推論は右辺の値から自動的に決定される

### 整数型

```zig
// 符号付き整数
const i8_val: i8 = -128;        // 8bit: -128 to 127
const i16_val: i16 = -32768;    // 16bit
const i32_val: i32 = -2147483648; // 32bit
const i64_val: i64 = -9223372036854775808; // 64bit

// 符号なし整数
const u8_val: u8 = 255;         // 8bit: 0 to 255
const u16_val: u16 = 65535;     // 16bit
const u32_val: u32 = 4294967295; // 32bit
const u64_val: u64 = 18446744073709551615; // 64bit

// カスタムビット幅（Zigの特徴的な機能）
const i3_val: i3 = -4;          // 3bit符号付き: -4 to 3
const u3_val: u3 = 7;           // 3bit符号なし: 0 to 7
```

### 浮動小数点型

```zig
const f16_val: f16 = 1.5;      // 16bit浮動小数点
const f32_val: f32 = 3.14159;  // 32bit浮動小数点
const f64_val: f64 = 2.71828;  // 64bit浮動小数点
const f128_val: f128 = 1.414;  // 128bit浮動小数点
```

### 論理型と文字型

```zig
// 論理型
const is_true: bool = true;
const is_false: bool = false;

// 論理演算
const and_result = is_true and is_false; // false
const or_result = is_true or is_false;   // true
const not_result = !is_true;             // false

// 文字
const char_a: u8 = 'A';         // ASCII文字
const char_unicode: u21 = '🦎'; // Unicode文字（Zigのマスコット）
```

### 文字列

```zig
// 文字列リテラル
const greeting = "Hello, Zig!";

// 複数行文字列
const multiline =
    \\これは複数行の
    \\文字列です。
    \\各行の先頭の\\は削除されます。
;
```

### Optional型（nullableポインタ）

```zig
// Optional型は ? を使って宣言
const maybe_number: ?i32 = 42;
const empty_number: ?i32 = null;

// Optional型のチェック
if (maybe_number) |value| {
    std.debug.print("値があります: {d}\n", .{value});
} else {
    std.debug.print("値がありません\n", .{});
}
```

**重要:** Zigでは`null`による事故を防ぐため、nullになる可能性のある値は明示的に`?`を付けて宣言する必要があります。

### 型キャスト

```zig
// 暗黙的な拡張キャスト
const small_int: i8 = 100;
const big_int: i32 = small_int;

// 明示的な縮小キャスト
const big_value: i32 = 127;
const small_value: i8 = @intCast(big_value);

// 型変換関数
const float_val: f32 = 3.7;
const int_from_float: i32 = @intFromFloat(float_val);
const float_from_int: f32 = @floatFromInt(int_from_float);
```

---

## 制御フロー

### if文と式

```zig
const score = 85;

// 基本的なif文
if (score >= 90) {
    std.debug.print("優秀です！\n", .{});
} else if (score >= 70) {
    std.debug.print("良い成績です。\n", .{});
} else {
    std.debug.print("がんばりましょう。\n", .{});
}

// if式（値を返す）
const grade = if (score >= 90) "A" else if (score >= 80) "B" else "C";

// Optional型とif文の組み合わせ
const maybe_value: ?i32 = 42;
if (maybe_value) |value| {
    std.debug.print("値が存在します: {d}\n", .{value});
} else {
    std.debug.print("値がありません\n", .{});
}
```

**ポイント:**
- if文は文（statement）としても式（expression）としても使える
- Optional型のアンラップは `|variable|` 構文を使用

### while文

```zig
// 基本的なwhile文
var i: u32 = 0;
while (i < 5) {
    std.debug.print("{d} ", .{i});
    i += 1;
}

// 継続式付きwhile文
var index: usize = 0;
const numbers = [_]i32{ 1, 2, 3, 4, 5 };
while (index < numbers.len) : (index += 1) {
    std.debug.print("{d} ", .{numbers[index]});
}

// continueの使用
var counter: u32 = 0;
while (counter < 10) : (counter += 1) {
    if (counter % 2 != 0) continue;
    std.debug.print("{d} ", .{counter});
}
```

### for文

```zig
const fruits = [_][]const u8{ "apple", "banana", "cherry" };

// 基本的なfor文
for (fruits) |fruit| {
    std.debug.print("- {s}\n", .{fruit});
}

// インデックス付きfor文
for (fruits, 0..) |fruit, idx| {
    std.debug.print("{d}: {s}\n", .{ idx, fruit });
}

// 可変要素への参照
var mutable_numbers = [_]i32{ 1, 2, 3, 4, 5 };
for (&mutable_numbers) |*num| {
    num.* *= 2; // 各要素を2倍にする
}
```

### switch文

```zig
const day = 3;

// 基本的なswitch文
switch (day) {
    1 => std.debug.print("月曜日\n", .{}),
    2 => std.debug.print("火曜日\n", .{}),
    3 => std.debug.print("水曜日\n", .{}),
    6, 7 => std.debug.print("週末\n", .{}), // 複数の値
    else => std.debug.print("無効な日付\n", .{}),
}

// switch式（値を返す）
const day_type = switch (day) {
    1...5 => "平日",    // 範囲指定
    6, 7 => "週末",
    else => "無効",
};

// 文字に対するswitch
const letter = 'A';
switch (letter) {
    'A', 'E', 'I', 'O', 'U' => std.debug.print("母音\n", .{}),
    else => std.debug.print("子音\n", .{}),
}
```

### ラベル付きループ

```zig
// ラベル付きbreak
outer: for (0..3) |i| {
    for (0..3) |j| {
        if (i == 1 and j == 1) {
            break :outer; // 外側のループを終了
        }
        std.debug.print("({d}, {d}) ", .{ i, j });
    }
}

// ラベル付きcontinue
outer_continue: for (0..3) |i| {
    for (0..3) |j| {
        if (j == 1) {
            continue :outer_continue; // 外側のループの次の反復へ
        }
        std.debug.print("({d}, {d}) ", .{ i, j });
    }
}
```

### defer文

```zig
{
    std.debug.print("処理開始\n", .{});
    defer std.debug.print("処理終了（defer）\n", .{});
    defer std.debug.print("cleanup処理（defer）\n", .{});
    
    std.debug.print("メイン処理\n", .{});
    // スコープを抜ける際に、defer文が逆順で実行される
}
```

**重要:** `defer`文はスコープを抜ける際に逆順で実行されます。リソースの解放などに使用されます。

---

## 関数

### 基本的な関数定義

```zig
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
```

### 引数の種類

```zig
// 値渡し
fn modify_value(value: i32) void {
    _ = value + 100; // valueは変更されない（ローカルコピー）
}

// 参照渡し（ポインタ）
fn modify_reference(value: *i32) void {
    value.* += 100; // 元の値が変更される
}

// 配列を受け取る関数
fn print_array(arr: *const [5]i32) void {
    for (arr) |value| {
        std.debug.print("{d} ", .{value});
    }
    std.debug.print("\n", .{});
}

// スライスを受け取る関数
fn print_slice(slice: []const i32) void {
    for (slice) |value| {
        std.debug.print("{d} ", .{value});
    }
    std.debug.print("\n", .{});
}
```

### 可変長引数

```zig
// 可変長引数関数
fn print_numbers(args: anytype) void {
    std.debug.print("数値: ", .{});
    inline for (args) |arg| {
        std.debug.print("{d} ", .{arg});
    }
    std.debug.print("\n", .{});
}

// 使用例
print_numbers(.{ 1, 2, 3 });
print_numbers(.{ 10, 20, 30, 40, 50 });
```

### 関数ポインタ

```zig
// 関数ポインタの宣言と使用
const add_func = add;
const result = add_func(5, 7);

// 関数を引数として受け取る関数
fn apply_operation(a: i32, b: i32, operation: fn (i32, i32) i32) i32 {
    return operation(a, b);
}

const add_result = apply_operation(10, 5, add);
const mult_result = apply_operation(10, 5, multiply);
```

### ジェネリック関数

```zig
// 型パラメータを持つ関数
fn max(comptime T: type, a: T, b: T) T {
    return if (a > b) a else b;
}

const max_int = max(i32, 10, 20);
const max_float = max(f32, 3.14, 2.71);

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
```

### 再帰関数

```zig
// 階乗
fn factorial(n: u32) u32 {
    if (n <= 1) return 1;
    return n * factorial(n - 1);
}

// フィボナッチ数列
fn fibonacci(n: u32) u32 {
    if (n <= 1) return n;
    return fibonacci(n - 1) + fibonacci(n - 2);
}
```

### エラーを返す関数

```zig
const DivisionError = error{
    DivisionByZero,
};

fn divide(a: i32, b: i32) DivisionError!i32 {
    if (b == 0) return DivisionError.DivisionByZero;
    return @divTrunc(a, b);
}

// 使用例
if (divide(10, 2)) |result| {
    std.debug.print("結果: {d}\n", .{result});
} else |err| {
    std.debug.print("エラー: {}\n", .{err});
}
```

---

## 構造体とEnum

### 基本的な構造体

```zig
const Person = struct {
    name: []const u8,
    age: u32,
    height: f32,

    // メソッド
    pub fn introduce(self: Person) void {
        std.debug.print("私の名前は{s}です。{d}歳です。\n", .{ self.name, self.age });
    }

    pub fn is_adult(self: Person) bool {
        return self.age >= 18;
    }
};

// 使用例
const person = Person{
    .name = "Alice",
    .age = 30,
    .height = 165.5,
};

person.introduce();
const adult = person.is_adult();
```

### 可変な構造体

```zig
const Counter = struct {
    value: i32,

    pub fn increment(self: *Counter) void {
        self.value += 1;
    }

    pub fn add(self: *Counter, amount: i32) void {
        self.value += amount;
    }

    pub fn reset(self: *Counter) void {
        self.value = 0;
    }
};

// 使用例
var counter = Counter{ .value = 0 };
counter.increment();
counter.add(5);
counter.reset();
```

### ネストした構造体

```zig
const Address = struct {
    street: []const u8,
    city: []const u8,
    zip_code: []const u8,
};

const Employee = struct {
    id: u32,
    person: Person,
    address: Address,
    salary: u32,

    pub fn display_info(self: Employee) void {
        std.debug.print("従業員ID: {d}\n", .{self.id});
        std.debug.print("名前: {s}, 年齢: {d}\n", .{ self.person.name, self.person.age });
        std.debug.print("住所: {s}, {s} {s}\n", .{ self.address.street, self.address.city, self.address.zip_code });
        std.debug.print("給与: {d}円\n", .{self.salary});
    }
};
```

### Enum

```zig
// 基本的なenum
const Day = enum {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
};

fn print_day(day: Day) void {
    switch (day) {
        .Monday => std.debug.print("月曜日\n", .{}),
        .Tuesday => std.debug.print("火曜日\n", .{}),
        .Wednesday => std.debug.print("水曜日\n", .{}),
        // ... 他の曜日
        else => std.debug.print("その他\n", .{}),
    }
}

// 値付きenum
const Color = enum(u32) {
    Red = 0xFF0000,
    Green = 0x00FF00,
    Blue = 0x0000FF,
};

const red = Color.Red;
const red_value = @intFromEnum(red); // 0xFF0000
```

### Union型

```zig
// Tagged Union
const Value = union(enum) {
    number: i32,
    text: []const u8,
    flag: bool,
};

fn print_value(value: Value) void {
    switch (value) {
        .number => |num| std.debug.print("数値: {d}\n", .{num}),
        .text => |txt| std.debug.print("テキスト: {s}\n", .{txt}),
        .flag => |flg| std.debug.print("フラグ: {}\n", .{flg}),
    }
}

// 使用例
const number_value = Value{ .number = 42 };
const text_value = Value{ .text = "Hello" };
print_value(number_value);
print_value(text_value);
```

### ジェネリック構造体

```zig
fn Box(comptime T: type) type {
    return struct {
        value: T,

        const Self = @This();

        pub fn get(self: Self) T {
            return self.value;
        }

        pub fn set(self: *Self, new_value: T) void {
            self.value = new_value;
        }
    };
}

// 使用例
var int_box = Box(i32){ .value = 42 };
var string_box = Box([]const u8){ .value = "Hello" };

std.debug.print("整数ボックス: {d}\n", .{int_box.get()});
int_box.set(100);
```

---

## エラーハンドリング

### エラー型の定義

```zig
// 基本的なエラー型
const MathError = error{
    DivisionByZero,
    Overflow,
    InvalidInput,
};

// エラーセットの組み合わせ
const ValidationError = error{
    EmptyName,
    NameTooLong,
} || AgeError;

const AgeError = error{
    TooYoung,
    TooOld,
    InvalidAge,
};
```

### エラーを返す関数

```zig
fn divide(a: i32, b: i32) MathError!i32 {
    if (b == 0) return MathError.DivisionByZero;
    return @divTrunc(a, b);
}

fn check_age(age: i32) AgeError![]const u8 {
    if (age < 0) return AgeError.InvalidAge;
    if (age < 18) return AgeError.TooYoung;
    if (age > 120) return AgeError.TooOld;
    
    return if (age < 65) "成人" else "シニア";
}
```

### try構文とcatch構文

```zig
// try構文 - エラーを上位に伝播
const safe_result = try divide(20, 4);

// catch構文 - エラーをキャッチして処理
const result = divide(15, 0) catch |err| {
    std.debug.print("エラーをキャッチしました: {}\n", .{err});
    return;
};

// デフォルト値を返すcatch
const safe_value = divide(10, 0) catch 0;
```

### defer文とerrdefer文

```zig
fn open_and_process_file(filename: []const u8) FileError!void {
    std.debug.print("ファイル '{s}' を開いています...\n", .{filename});
    defer std.debug.print("ファイル '{s}' を閉じます\n", .{filename});

    // ファイルが存在しない場合のシミュレーション
    if (std.mem.eql(u8, filename, "missing.txt")) {
        return FileError.FileNotFound;
    }

    std.debug.print("ファイル '{s}' を処理中...\n", .{filename});
}

// errdefer - エラーが発生した場合のみ実行
fn create_resource() !void {
    std.debug.print("リソースを作成中...\n", .{});
    errdefer std.debug.print("エラーが発生したため、リソースをクリーンアップします\n", .{});

    // エラーが発生した場合のシミュレーション
    if (setup_failed) {
        return error.SetupFailed;
    }
}
```

### Optional型とエラーの組み合わせ

```zig
// エラーとOptional型の組み合わせ
fn read_file_safely(filename: []const u8) FileError!?[]const u8 {
    if (std.mem.eql(u8, filename, "missing.txt")) {
        return FileError.FileNotFound;
    }
    
    if (std.mem.eql(u8, filename, "empty.txt")) {
        return null; // ファイルは存在するが空
    }
    
    return "ファイル内容";
}

// 使用例
if (read_file_safely("config.txt")) |content| {
    if (content) |data| {
        std.debug.print("ファイル内容: {s}\n", .{data});
    } else {
        std.debug.print("ファイルは空です\n", .{});
    }
} else |err| {
    std.debug.print("ファイル読み込みエラー: {}\n", .{err});
}
```

---

## メモリ管理

### アロケータの基本

```zig
// 標準的なアロケータの取得
var gpa = std.heap.GeneralPurposeAllocator(.{}){};
defer _ = gpa.deinit();
const allocator = gpa.allocator();

// 単一の値をアロケート
const single_value = try allocator.create(i32);
defer allocator.destroy(single_value);

single_value.* = 42;

// 配列のアロケート
const array = try allocator.alloc(i32, 5);
defer allocator.free(array);

// 配列に値を設定
for (array, 0..) |*item, i| {
    item.* = @intCast(i * 10);
}
```

### 動的配列（ArrayList）

```zig
const ArrayList = std.ArrayList;

// ArrayListの作成
var list = ArrayList(i32).init(allocator);
defer list.deinit();

// 要素の追加
try list.append(10);
try list.append(20);
try list.append(30);

// 要素の挿入と削除
try list.insert(1, 15);
_ = list.swapRemove(2);

// 要素へのアクセス
for (list.items) |item| {
    std.debug.print("{d} ", .{item});
}
```

### 文字列のメモリ管理

```zig
// 動的文字列の作成
const dynamic_string = try allocator.alloc(u8, 20);
defer allocator.free(dynamic_string);

// 文字列のコピー
const source = "Hello, Zig!";
@memcpy(dynamic_string[0..source.len], source);

// 文字列の複製
const duplicated = try allocator.dupe(u8, source);
defer allocator.free(duplicated);
```

### 異なるアロケータの種類

```zig
// アリーナアロケータ - 一括解放
var arena = std.heap.ArenaAllocator.init(allocator);
defer arena.deinit(); // すべてのメモリを一括解放
const arena_allocator = arena.allocator();

const arena_array = try arena_allocator.alloc(i32, 10);
// 個別のfreeは不要

// FixedBufferAllocator - 固定サイズバッファ
var buffer: [1024]u8 = undefined;
var fba = std.heap.FixedBufferAllocator.init(&buffer);
const fixed_allocator = fba.allocator();

const fixed_array = try fixed_allocator.alloc(i32, 10);
defer fixed_allocator.free(fixed_array);
```

### RAIIパターン

```zig
const ManagedResource = struct {
    allocator: Allocator,
    data: []u8,

    pub fn init(allocator: Allocator, initial_data: []const u8) !ManagedResource {
        const data_copy = try allocator.dupe(u8, initial_data);
        return ManagedResource{
            .allocator = allocator,
            .data = data_copy,
        };
    }

    pub fn deinit(self: *ManagedResource) void {
        self.allocator.free(self.data);
    }

    pub fn modify(self: *ManagedResource, new_data: []const u8) !void {
        const new_data_copy = try self.allocator.dupe(u8, new_data);
        self.allocator.free(self.data);
        self.data = new_data_copy;
    }
};

// 使用例
var resource = try ManagedResource.init(allocator, "初期データ");
defer resource.deinit();

try resource.modify("更新されたデータ");
```

---

## 配列とスライス

### 基本的な配列

```zig
// 固定長配列の宣言と初期化
const numbers = [5]i32{ 1, 2, 3, 4, 5 };

// 型推論を使った配列
const auto_array = [_]i32{ 10, 20, 30, 40 };

// すべて同じ値で初期化
const zeros = [_]i32{0} ** 5;

// 未初期化配列（使用前に値を設定する必要がある）
var uninitialized: [3]i32 = undefined;
uninitialized[0] = 100;
uninitialized[1] = 200;
uninitialized[2] = 300;
```

### 多次元配列

```zig
// 2次元配列
const matrix = [3][3]i32{
    [_]i32{ 1, 2, 3 },
    [_]i32{ 4, 5, 6 },
    [_]i32{ 7, 8, 9 },
};

// アクセス
for (matrix, 0..) |row, i| {
    for (row, 0..) |value, j| {
        std.debug.print("matrix[{d}][{d}] = {d}\n", .{ i, j, value });
    }
}
```

### スライスの基本

```zig
const source_array = [_]i32{ 10, 20, 30, 40, 50, 60, 70, 80, 90, 100 };

// 基本的なスライス
const slice1 = source_array[2..5]; // インデックス2から4まで
const slice2 = source_array[7..];  // インデックス7から最後まで
const slice3 = source_array[..3];  // 最初からインデックス2まで
const full_slice = source_array[0..source_array.len]; // 全体

// スライスの要素を変更
var mutable_array = [_]i32{ 1, 2, 3, 4, 5, 6, 7, 8 };
const mutable_slice = mutable_array[2..6];

for (mutable_slice) |*value| {
    value.* *= 10; // 各要素を10倍
}
```

### 文字列とスライス

```zig
const hello = "Hello, Zig!";

// 文字列のスライス
const greeting = hello[0..5]; // "Hello"
const target = hello[7..10];  // "Zig"

// UTF-8エンコーディングの処理
const japanese = "こんにちは、Zig！";
const hello_part = japanese[0..15]; // "こんにちは" (注意: バイト単位)
```

### 配列操作関数

```zig
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

// バブルソート
fn bubble_sort(slice: []i32) void {
    const n = slice.len;
    var i: usize = 0;
    while (i < n) : (i += 1) {
        var j: usize = 0;
        while (j < n - 1 - i) : (j += 1) {
            if (slice[j] > slice[j + 1]) {
                const temp = slice[j];
                slice[j] = slice[j + 1];
                slice[j + 1] = temp;
            }
        }
    }
}
```

### 配列の連結とコピー

```zig
const array1 = [_]i32{ 1, 2, 3 };
const array2 = [_]i32{ 4, 5, 6 };

// 配列の連結（コンパイル時）
const concatenated = array1 ++ array2;

// 配列のコピー
var copied_array: [6]i32 = undefined;
@memcpy(copied_array[0..3], &array1);
@memcpy(copied_array[3..6], &array2);
```

### センチネル終端配列

```zig
// null終端文字列（C文字列互換）
const c_string: [*:0]const u8 = "Hello from C!";

// センチネル値付き配列
const sentinel_array = [_:0]i32{ 1, 2, 3, 4, 5 };
std.debug.print("センチネル値: {d}\n", .{sentinel_array[sentinel_array.len]});
```

---

## コンパイル時機能

### 基本的なcomptime

```zig
// コンパイル時に計算される定数
const compile_time_result = comptime fibonacci(10);

// コンパイル時の型生成
const IntArray = comptime create_array_type(i32, 5);
const int_array: IntArray = [_]i32{ 1, 2, 3, 4, 5 };

fn create_array_type(comptime T: type, comptime size: usize) type {
    return [size]T;
}
```

### comptime引数

```zig
// ジェネリック関数でのcomptime使用
fn generic_add(comptime T: type, a: T, b: T) T {
    return a + b;
}

const result1 = generic_add(i32, 10, 20);
const result2 = generic_add(f32, 3.14, 2.86);

// 配列サイズをcomptime引数として受け取る
fn create_initialized_array(comptime size: usize, value: i32) [size]i32 {
    return [_]i32{value} ** size;
}

const array = create_initialized_array(5, 42);
```

### コンパイル時の型操作

```zig
// 型情報の取得と表示
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

// 使用例
print_type_info(@TypeOf(42));
print_type_info(@TypeOf(3.14));
```

### コンパイル時のループ

```zig
// コンパイル時に複数の型で同じ処理を実行
const types = [_]type{ i8, i16, i32, i64, u8, u16, u32, u64 };

std.debug.print("各整数型のサイズ:\n", .{});
inline for (types) |T| {
    std.debug.print("{s}: {d} bytes\n", .{ @typeName(T), @sizeOf(T) });
}
```

### メタプログラミング

```zig
// 構造体の自動生成
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

const Point2D = create_point_type(f32, 2);
const Point3D = create_point_type(f64, 3);

const p2d = Point2D{ .x = 1.0, .y = 2.0 };
const p3d = Point3D{ .x = 1.0, .y = 2.0, .z = 3.0 };
```

### コンパイル時の条件分岐

```zig
// プラットフォーム固有のコード
fn get_platform_info() []const u8 {
    return switch (@import("builtin").target.os.tag) {
        .windows => "Windows",
        .macos => "macOS",
        .linux => "Linux",
        else => "Unknown",
    };
}

const platform_info = comptime get_platform_info();

// デバッグビルド専用のコード
if (comptime std.debug.runtime_safety) {
    std.debug.print("デバッグビルドで実行中\n", .{});
} else {
    std.debug.print("リリースビルドで実行中\n", .{});
}
```

### ビルド時設定

```zig
const BuildConfig = struct {
    debug_mode: bool,
    optimization_level: u8,
    feature_flags: FeatureFlags,

    const FeatureFlags = struct {
        networking: bool,
        graphics: bool,
    };
};

const config = comptime BuildConfig{
    .debug_mode = true,
    .optimization_level = 2,
    .feature_flags = .{ .networking = true, .graphics = false },
};

if (comptime config.debug_mode) {
    std.debug.print("デバッグモードが有効です\n", .{});
}
```

### コンパイル時テスト

```zig
// コンパイル時にテストを実行
comptime {
    const test_result = fibonacci(5);
    if (test_result != 5) {
        @compileError("fibonacci(5) should return 5");
    }
}

test "コンパイル時計算のテスト" {
    const result = comptime fibonacci(7);
    try std.testing.expect(result == 13);
}
```

---

## 高度な機能

### ポインタと参照

```zig
// 基本的なポインタ操作
var value: i32 = 42;
const ptr: *i32 = &value;

std.debug.print("値: {d}, ポインタ経由: {d}\n", .{ value, ptr.* });

// ポインタを通じて値を変更
ptr.* = 100;
std.debug.print("変更後の値: {d}\n", .{value});

// 配列のポインタ
var array = [_]i32{ 1, 2, 3, 4, 5 };
const array_ptr: *[5]i32 = &array;

// スライスポインタ
const slice_ptr: [*]i32 = &array[0];
std.debug.print("スライスポインタ経由: {d}\n", .{slice_ptr[2]});
```

### インライン関数とインラインアセンブリ

```zig
// インライン関数
inline fn square(x: i32) i32 {
    return x * x;
}

// インラインアセンブリ（アーキテクチャ依存）
fn get_cpu_id() u32 {
    return switch (@import("builtin").target.cpu.arch) {
        .x86_64 => asm volatile ("cpuid"
            : [ret] "={eax}" (-> u32),
            : [input] "{eax}" (0),
            : "ebx", "ecx", "edx"
        ),
        else => 0,
    };
}
```

### パックド構造体

```zig
// ビットレベルでの制御が必要な場合
const PackedStruct = packed struct {
    flag1: bool,
    flag2: bool,
    flag3: bool,
    value: u5,  // 5bit値
};

const packed_value = PackedStruct{
    .flag1 = true,
    .flag2 = false,
    .flag3 = true,
    .value = 15,
};

std.debug.print("パックド構造体のサイズ: {d} bytes\n", .{@sizeOf(PackedStruct)});
```

### 実践的なデータ構造例

```zig
// 二分探索木（ジェネリック）
fn BinarySearchTree(comptime T: type) type {
    return struct {
        const Self = @This();
        
        const Node = struct {
            value: T,
            left: ?*Node,
            right: ?*Node,
        };
        
        root: ?*Node,
        allocator: Allocator,
        
        pub fn init(allocator: Allocator) Self {
            return Self{
                .root = null,
                .allocator = allocator,
            };
        }
        
        pub fn insert(self: *Self, value: T) !void {
            self.root = try self.insert_node(self.root, value);
        }
        
        fn insert_node(self: *Self, node: ?*Node, value: T) !?*Node {
            if (node == null) {
                const new_node = try self.allocator.create(Node);
                new_node.* = Node{
                    .value = value,
                    .left = null,
                    .right = null,
                };
                return new_node;
            }
            
            if (value < node.?.value) {
                node.?.left = try self.insert_node(node.?.left, value);
            } else if (value > node.?.value) {
                node.?.right = try self.insert_node(node.?.right, value);
            }
            
            return node;
        }
        
        pub fn search(self: *const Self, value: T) bool {
            return self.search_node(self.root, value);
        }
        
        fn search_node(self: *const Self, node: ?*Node, value: T) bool {
            if (node == null) return false;
            
            if (value == node.?.value) return true;
            if (value < node.?.value) return self.search_node(node.?.left, value);
            return self.search_node(node.?.right, value);
        }
    };
}
```

---

## 重要なポイントと他言語との違い

### メモリ安全性
- **明示的なメモリ管理**: アロケータを明示的に指定
- **null安全性**: Optional型（`?`）による明示的なnull処理
- **境界チェック**: 配列アクセスの境界チェック（デバッグモード）

### エラーハンドリング
- **例外なし**: try/catchではなくエラーユニオン型（`!`）を使用
- **明示的なエラー伝播**: `try`キーワードによる明示的なエラー伝播
- **コンパイル時チェック**: 未処理のエラーはコンパイルエラー

### コンパイル時計算
- **強力なメタプログラミング**: `comptime`による高度なコンパイル時計算
- **型レベル計算**: 型も値として扱える
- **ゼロコスト抽象化**: 多くの抽象化がランタイムコストゼロ

### 制御フロー
- **隠れた制御フローなし**: 例外処理やガベージコレクションなし
- **明示的な全て**: 暗黙的な型変換や隠れた処理なし
- **予測可能な動作**: パフォーマンスが予測しやすい

---

このガイドは、Zigの主要な機能と文法を包括的にカバーしています。各サンプルファイルには詳細なコメントが含まれているので、実際のコードと合わせて学習することをお勧めします。