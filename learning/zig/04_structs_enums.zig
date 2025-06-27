const std = @import("std");

pub fn main() !void {
    // ==========================================
    // 構造体とenum
    // ==========================================
    
    std.debug.print("=== Zigの構造体とenum ===\n");
    
    // ------------------------------------------
    // 基本的な構造体
    // ------------------------------------------
    
    std.debug.print("\n=== 基本的な構造体 ===\n");
    
    // 構造体のインスタンス作成
    const person = Person{
        .name = "Alice",
        .age = 30,
        .height = 165.5,
    };
    
    std.debug.print("名前: {s}, 年齢: {d}, 身長: {d}cm\n", .{ person.name, person.age, person.height });
    
    // 構造体のメソッド呼び出し
    person.introduce();
    const is_adult = person.is_adult();
    std.debug.print("成人かどうか: {}\n", .{is_adult});
    
    // ------------------------------------------
    // 可変な構造体
    // ------------------------------------------
    
    std.debug.print("\n=== 可変な構造体 ===\n");
    
    var counter = Counter{ .value = 0 };
    std.debug.print("初期値: {d}\n", .{counter.value});
    
    counter.increment();
    counter.increment();
    std.debug.print("インクリメント後: {d}\n", .{counter.value});
    
    counter.add(5);
    std.debug.print("5を追加後: {d}\n", .{counter.value});
    
    counter.reset();
    std.debug.print("リセット後: {d}\n", .{counter.value});
    
    // ------------------------------------------
    // ネストした構造体
    // ------------------------------------------
    
    std.debug.print("\n=== ネストした構造体 ===\n");
    
    const address = Address{
        .street = "123 Main St",
        .city = "Tokyo",
        .zip_code = "100-0001",
    };
    
    const employee = Employee{
        .id = 1001,
        .person = Person{
            .name = "Bob",
            .age = 25,
            .height = 175.0,
        },
        .address = address,
        .salary = 50000,
    };
    
    employee.display_info();
    
    // ------------------------------------------
    // 基本的なenum
    // ------------------------------------------
    
    std.debug.print("\n=== 基本的なenum ===\n");
    
    const today = Day.Wednesday;
    std.debug.print("今日は: ");
    print_day(today);
    
    const tomorrow = Day.Thursday;
    if (is_weekend(tomorrow)) {
        std.debug.print("明日は週末です！\n");
    } else {
        std.debug.print("明日は平日です。\n");
    }
    
    // ------------------------------------------
    // 値付きenum
    // ------------------------------------------
    
    std.debug.print("\n=== 値付きenum ===\n");
    
    const red = Color.Red;
    const green = Color.Green;
    const blue = Color.Blue;
    
    std.debug.print("色の値: Red={d}, Green={d}, Blue={d}\n", .{ @intFromEnum(red), @intFromEnum(green), @intFromEnum(blue) });
    
    // ------------------------------------------
    // Union型
    // ------------------------------------------
    
    std.debug.print("\n=== Union型 ===\n");
    
    var number_value = Value{ .number = 42 };
    var text_value = Value{ .text = "Hello" };
    var flag_value = Value{ .flag = true };
    
    print_value(number_value);
    print_value(text_value);
    print_value(flag_value);
    
    // ------------------------------------------
    // Tagged Union
    // ------------------------------------------
    
    std.debug.print("\n=== Tagged Union ===\n");
    
    const int_result = Result{ .success = 100 };
    const error_result = Result{ .error_msg = "ファイルが見つかりません" };
    
    handle_result(int_result);
    handle_result(error_result);
    
    // ------------------------------------------
    // ジェネリック構造体
    // ------------------------------------------
    
    std.debug.print("\n=== ジェネリック構造体 ===\n");
    
    var int_box = Box(i32){ .value = 42 };
    var string_box = Box([]const u8){ .value = "Hello, Zig!" };
    
    std.debug.print("整数ボックス: {d}\n", .{int_box.get()});
    std.debug.print("文字列ボックス: {s}\n", .{string_box.get()});
    
    int_box.set(100);
    string_box.set("Updated!");
    
    std.debug.print("更新後 - 整数ボックス: {d}\n", .{int_box.get()});
    std.debug.print("更新後 - 文字列ボックス: {s}\n", .{string_box.get()});
    
    // ------------------------------------------
    // Optional型とUnion
    // ------------------------------------------
    
    std.debug.print("\n=== Optional型とUnion ===\n");
    
    const found_person = find_person("Alice");
    if (found_person) |p| {
        std.debug.print("見つかった人: {s}, {d}歳\n", .{ p.name, p.age });
    } else {
        std.debug.print("人が見つかりませんでした\n");
    }
    
    const not_found = find_person("Charlie");
    if (not_found) |p| {
        std.debug.print("見つかった人: {s}, {d}歳\n", .{ p.name, p.age });
    } else {
        std.debug.print("人が見つかりませんでした\n");
    }
}

// ------------------------------------------
// 構造体定義
// ------------------------------------------

// 基本的な構造体
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
    
    pub fn get_bmi(self: Person, weight: f32) f32 {
        const height_m = self.height / 100.0;
        return weight / (height_m * height_m);
    }
};

// 可変なフィールドを持つ構造体
const Counter = struct {
    value: i32,
    
    pub fn increment(self: *Counter) void {
        self.value += 1;
    }
    
    pub fn decrement(self: *Counter) void {
        self.value -= 1;
    }
    
    pub fn add(self: *Counter, amount: i32) void {
        self.value += amount;
    }
    
    pub fn reset(self: *Counter) void {
        self.value = 0;
    }
};

// ネストした構造体
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
    
    pub fn get_annual_salary(self: Employee) u32 {
        return self.salary * 12;
    }
};

// ------------------------------------------
// enum定義
// ------------------------------------------

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
        .Monday => std.debug.print("月曜日\n"),
        .Tuesday => std.debug.print("火曜日\n"),
        .Wednesday => std.debug.print("水曜日\n"),
        .Thursday => std.debug.print("木曜日\n"),
        .Friday => std.debug.print("金曜日\n"),
        .Saturday => std.debug.print("土曜日\n"),
        .Sunday => std.debug.print("日曜日\n"),
    }
}

fn is_weekend(day: Day) bool {
    return day == .Saturday or day == .Sunday;
}

// 値付きenum
const Color = enum(u8) {
    Red = 0xFF0000,
    Green = 0x00FF00,
    Blue = 0x0000FF,
};

// ------------------------------------------
// Union型定義
// ------------------------------------------

// 基本的なUnion
const Value = union {
    number: i32,
    text: []const u8,
    flag: bool,
};

fn print_value(value: Value) void {
    // Unionの場合、どのフィールドがアクティブかを知る方法がない
    // 通常はTagged Unionを使う
    std.debug.print("値が設定されています（型不明）\n");
}

// Tagged Union
const Result = union(enum) {
    success: i32,
    error_msg: []const u8,
};

fn handle_result(result: Result) void {
    switch (result) {
        .success => |value| std.debug.print("成功: {d}\n", .{value}),
        .error_msg => |msg| std.debug.print("エラー: {s}\n", .{msg}),
    }
}

// ------------------------------------------
// ジェネリック構造体
// ------------------------------------------

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
        
        pub fn is_equal(self: Self, other: Self) bool {
            return self.value == other.value;
        }
    };
}

// ------------------------------------------
// Optional型を返す関数
// ------------------------------------------

fn find_person(name: []const u8) ?Person {
    // 簡単なデータベースシミュレーション
    const people = [_]Person{
        Person{ .name = "Alice", .age = 30, .height = 165.5 },
        Person{ .name = "Bob", .age = 25, .height = 175.0 },
    };
    
    for (people) |person| {
        if (std.mem.eql(u8, person.name, name)) {
            return person;
        }
    }
    
    return null;
}