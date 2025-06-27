const std = @import("std");

pub fn main() !void {
    // ==========================================
    // エラーハンドリング
    // ==========================================

    std.debug.print("=== Zigのエラーハンドリング ===\n", .{});

    // ------------------------------------------
    // 基本的なエラーハンドリング
    // ------------------------------------------

    std.debug.print("\n=== 基本的なエラーハンドリング ===\n", .{});

    // エラーを返す可能性のある関数の呼び出し
    const result1 = divide(10, 2);
    if (result1) |value| {
        std.debug.print("10 / 2 = {d}\n", .{value});
    } else |err| {
        std.debug.print("エラーが発生しました: {}\n", .{err});
    }

    // ゼロ除算のエラーケース
    const result2 = divide(10, 0);
    if (result2) |value| {
        std.debug.print("10 / 0 = {d}\n", .{value});
    } else |err| {
        std.debug.print("エラーが発生しました: {}\n", .{err});
    }

    // ------------------------------------------
    // try構文
    // ------------------------------------------

    std.debug.print("\n=== try構文 ===\n", .{});

    // try構文を使ったエラーハンドリング
    // エラーが発生した場合、呼び出し元にエラーを伝播
    const safe_result = try divide(20, 4);
    std.debug.print("20 / 4 = {d}\n", .{safe_result});

    // catch構文でエラーをキャッチ
    const caught_result = divide(15, 0) catch |err| {
        std.debug.print("エラーをキャッチしました: {}\n", .{err});
        return; // エラーの場合は関数を終了
    };
    std.debug.print("15 / 0 = {d}\n", .{caught_result});

    // ------------------------------------------
    // カスタムエラー型
    // ------------------------------------------

    std.debug.print("\n=== カスタムエラー型 ===\n", .{});

    // 年齢チェック関数の使用例
    const ages = [_]i32{ 25, 17, -5, 150 };

    for (ages) |age| {
        const category = check_age(age) catch |err| {
            std.debug.print("年齢 {d}: エラー - {}\n", .{ age, err });
            continue;
        };
        std.debug.print("年齢 {d}: {s}\n", .{ age, category });
    }

    // ------------------------------------------
    // エラーセットの組み合わせ
    // ------------------------------------------

    std.debug.print("\n=== エラーセットの組み合わせ ===\n", .{});

    // 複数のエラーを返す可能性のある関数
    const validation_tests = [_]struct { name: []const u8, age: i32 }{
        .{ .name = "Alice", .age = 25 },
        .{ .name = "", .age = 30 },
        .{ .name = "Bob", .age = -5 },
        .{ .name = "Charlie", .age = 200 },
    };

    for (validation_tests) |test_case| {
        const result = validate_person(test_case.name, test_case.age) catch |err| {
            std.debug.print("バリデーションエラー - 名前: '{s}', 年齢: {d} - {}\n", .{ test_case.name, test_case.age, err });
            continue;
        };
        std.debug.print("バリデーション成功: {s}\n", .{result});
    }

    // ------------------------------------------
    // defer文とエラーハンドリング
    // ------------------------------------------

    std.debug.print("\n=== defer文とエラーハンドリング ===\n", .{});

    // リソースの確保と解放
    open_and_process_file("example.txt") catch |err| {
        std.debug.print("ファイル処理エラー: {}\n", .{err});
    };

    // ------------------------------------------
    // errdefer文
    // ------------------------------------------

    std.debug.print("\n=== errdefer文 ===\n", .{});

    // エラーが発生した場合のみクリーンアップを実行
    create_and_setup_resource() catch |err| {
        std.debug.print("リソース作成エラー: {}\n", .{err});
    };

    // ------------------------------------------
    // Optional型とエラーの組み合わせ
    // ------------------------------------------

    std.debug.print("\n=== Optional型とエラーの組み合わせ ===\n", .{});

    // Optional型を返す関数（nullまたは値）
    const search_queries = [_][]const u8{ "apple", "banana", "grape", "orange" };

    for (search_queries) |query| {
        if (search_item(query)) |item| {
            std.debug.print("'{s}' が見つかりました: {s}\n", .{ query, item });
        } else {
            std.debug.print("'{s}' は見つかりませんでした\n", .{query});
        }
    }

    // エラーとOptional型の組み合わせ
    const file_paths = [_][]const u8{ "config.txt", "data.json", "missing.txt" };

    for (file_paths) |path| {
        if (read_file_safely(path)) |content| {
            if (content) |data| {
                std.debug.print("ファイル '{s}' の内容: {s}\n", .{ path, data });
            } else {
                std.debug.print("ファイル '{s}' は空です\n", .{path});
            }
        } else |err| {
            std.debug.print("ファイル '{s}' の読み込みエラー: {}\n", .{ path, err });
        }
    }

    // ------------------------------------------
    // エラーの詳細情報
    // ------------------------------------------

    std.debug.print("\n=== エラーの詳細情報 ===\n", .{});

    // エラーの詳細情報を含む構造体
    const parsing_tests = [_][]const u8{ "123", "abc", "45.67", "" };

    for (parsing_tests) |test_str| {
        const result = parse_number_with_details(test_str) catch |err| {
            std.debug.print("パースエラー '{s}': {}\n", .{ test_str, err });
            continue;
        };
        std.debug.print("パース成功 '{s}': {d}\n", .{ test_str, result });
    }
}

// ------------------------------------------
// エラー型の定義
// ------------------------------------------

// 基本的なエラー型
const MathError = error{
    DivisionByZero,
    Overflow,
    InvalidInput,
};

// 年齢チェック用のエラー型
const AgeError = error{
    TooYoung,
    TooOld,
    InvalidAge,
};

// バリデーション用のエラー型（複数のエラーセットを組み合わせ）
const ValidationError = error{
    EmptyName,
    NameTooLong,
} || AgeError;

// ファイル処理用のエラー型
const FileError = error{
    FileNotFound,
    PermissionDenied,
    ReadError,
    WriteError,
};

// パース用のエラー型
const ParseError = error{
    InvalidFormat,
    EmptyString,
    NumberTooLarge,
};

// ------------------------------------------
// エラーを返す関数
// ------------------------------------------

// 基本的な除算関数
fn divide(a: i32, b: i32) MathError!i32 {
    if (b == 0) return MathError.DivisionByZero;
    return @divTrunc(a, b);
}

// 年齢チェック関数
fn check_age(age: i32) AgeError![]const u8 {
    if (age < 0) return AgeError.InvalidAge;
    if (age < 18) return AgeError.TooYoung;
    if (age > 120) return AgeError.TooOld;

    return if (age < 65) "成人" else "シニア";
}

// 複数のエラーを返す可能性のあるバリデーション関数
fn validate_person(name: []const u8, age: i32) ValidationError![]const u8 {
    // 名前のバリデーション
    if (name.len == 0) return ValidationError.EmptyName;
    if (name.len > 50) return ValidationError.NameTooLong;

    // 年齢のバリデーション（AgeErrorを伝播）
    _ = try check_age(age);

    return "バリデーション成功";
}

// defer文を使ったリソース管理
fn open_and_process_file(filename: []const u8) FileError!void {
    std.debug.print("ファイル '{s}' を開いています...\n", .{filename});
    defer std.debug.print("ファイル '{s}' を閉じます\n", .{filename});

    // ファイルが存在しない場合のシミュレーション
    if (std.mem.eql(u8, filename, "missing.txt")) {
        return FileError.FileNotFound;
    }

    std.debug.print("ファイル '{s}' を処理中...\n", .{filename});

    // 処理中にエラーが発生した場合のシミュレーション
    if (std.mem.eql(u8, filename, "corrupted.txt")) {
        return FileError.ReadError;
    }

    std.debug.print("ファイル '{s}' の処理が完了しました\n", .{filename});
}

// errdefer文を使ったエラー時のクリーンアップ
fn create_and_setup_resource() !void {
    std.debug.print("リソースを作成中...\n", .{});
    errdefer std.debug.print("エラーが発生したため、リソースをクリーンアップします\n", .{});

    // リソース作成が成功したとする
    std.debug.print("リソースが作成されました\n", .{});

    // セットアップ中にエラーが発生した場合のシミュレーション
    const setup_failed = true;
    if (setup_failed) {
        std.debug.print("セットアップ中にエラーが発生しました\n", .{});
        return error.SetupFailed;
    }

    std.debug.print("リソースのセットアップが完了しました\n", .{});
}

// Optional型を返す関数
fn search_item(query: []const u8) ?[]const u8 {
    const items = [_][]const u8{ "apple", "banana", "cherry" };

    for (items) |item| {
        if (std.mem.eql(u8, item, query)) {
            return item;
        }
    }

    return null;
}

// エラーとOptional型の組み合わせ
fn read_file_safely(filename: []const u8) FileError!?[]const u8 {
    // ファイルが存在しない場合
    if (std.mem.eql(u8, filename, "missing.txt")) {
        return FileError.FileNotFound;
    }

    // ファイルが空の場合
    if (std.mem.eql(u8, filename, "empty.txt")) {
        return null;
    }

    // 通常のファイル
    if (std.mem.eql(u8, filename, "config.txt")) {
        return "設定データ";
    }

    if (std.mem.eql(u8, filename, "data.json")) {
        return "{\"key\": \"value\"}";
    }

    return FileError.FileNotFound;
}

// エラーの詳細情報を含む関数
fn parse_number_with_details(str: []const u8) ParseError!i32 {
    if (str.len == 0) return ParseError.EmptyString;

    // 数値以外の文字が含まれているかチェック
    for (str) |char| {
        if (char < '0' or char > '9') {
            return ParseError.InvalidFormat;
        }
    }

    // 簡単な数値パース（実際の実装では std.fmt.parseInt を使用）
    var result: i32 = 0;
    for (str) |char| {
        const digit: i32 = char - '0';
        if (result > @divTrunc(std.math.maxInt(i32) - digit, 10)) {
            return ParseError.NumberTooLarge;
        }
        result = result * 10 + digit;
    }

    return result;
}
