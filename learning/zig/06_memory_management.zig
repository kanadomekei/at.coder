const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

pub fn main() !void {
    // ==========================================
    // メモリ管理
    // ==========================================
    
    std.debug.print("=== Zigのメモリ管理 ===\n");
    
    // ------------------------------------------
    // アロケータの基本
    // ------------------------------------------
    
    std.debug.print("\n=== アロケータの基本 ===\n");
    
    // 標準的なアロケータの取得
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    // 単一の値をアロケート
    const single_value = try allocator.create(i32);
    defer allocator.destroy(single_value);
    
    single_value.* = 42;
    std.debug.print("アロケートした値: {d}\n", .{single_value.*});
    
    // ------------------------------------------
    // 配列のアロケート
    // ------------------------------------------
    
    std.debug.print("\n=== 配列のアロケート ===\n");
    
    // 固定サイズの配列をアロケート
    const array = try allocator.alloc(i32, 5);
    defer allocator.free(array);
    
    // 配列に値を設定
    for (array, 0..) |*item, i| {
        item.* = @intCast(i * 10);
    }
    
    std.debug.print("アロケートした配列: ");
    for (array) |item| {
        std.debug.print("{d} ", .{item});
    }
    std.debug.print("\n");
    
    // ------------------------------------------
    // 動的配列（ArrayList）
    // ------------------------------------------
    
    std.debug.print("\n=== 動的配列（ArrayList） ===\n");
    
    // ArrayListの作成
    var list = ArrayList(i32).init(allocator);
    defer list.deinit();
    
    // 要素の追加
    try list.append(10);
    try list.append(20);
    try list.append(30);
    
    std.debug.print("ArrayList要素数: {d}\n", .{list.items.len});
    std.debug.print("ArrayList容量: {d}\n", .{list.capacity});
    
    // 要素の表示
    std.debug.print("ArrayList内容: ");
    for (list.items) |item| {
        std.debug.print("{d} ", .{item});
    }
    std.debug.print("\n");
    
    // 要素の挿入と削除
    try list.insert(1, 15);
    std.debug.print("インデックス1に15を挿入: ");
    for (list.items) |item| {
        std.debug.print("{d} ", .{item});
    }
    std.debug.print("\n");
    
    _ = list.swapRemove(2);
    std.debug.print("インデックス2を削除: ");
    for (list.items) |item| {
        std.debug.print("{d} ", .{item});
    }
    std.debug.print("\n");
    
    // ------------------------------------------
    // 文字列のメモリ管理
    // ------------------------------------------
    
    std.debug.print("\n=== 文字列のメモリ管理 ===\n");
    
    // 動的文字列の作成
    const dynamic_string = try allocator.alloc(u8, 20);
    defer allocator.free(dynamic_string);
    
    // 文字列のコピー
    const source = "Hello, Zig!";
    @memcpy(dynamic_string[0..source.len], source);
    
    std.debug.print("動的文字列: {s}\n", .{dynamic_string[0..source.len]});
    
    // 文字列の複製
    const duplicated = try allocator.dupe(u8, source);
    defer allocator.free(duplicated);
    
    std.debug.print("複製された文字列: {s}\n", .{duplicated});
    
    // ------------------------------------------
    // カスタム構造体のメモリ管理
    // ------------------------------------------
    
    std.debug.print("\n=== カスタム構造体のメモリ管理 ===\n");
    
    // Person構造体のインスタンスをアロケート
    const person = try allocator.create(Person);
    defer allocator.destroy(person);
    
    person.* = Person{
        .name = try allocator.dupe(u8, "Alice"),
        .age = 30,
        .scores = ArrayList(i32).init(allocator),
    };
    defer person.deinit(allocator);
    
    // スコアの追加
    try person.scores.append(85);
    try person.scores.append(92);
    try person.scores.append(78);
    
    person.display();
    
    // ------------------------------------------
    // 異なるアロケータの種類
    // ------------------------------------------
    
    std.debug.print("\n=== 異なるアロケータの種類 ===\n");
    
    // アリーナアロケータ
    var arena = std.heap.ArenaAllocator.init(allocator);
    defer arena.deinit();
    const arena_allocator = arena.allocator();
    
    // アリーナアロケータでは個別のfreeが不要
    const arena_array1 = try arena_allocator.alloc(i32, 10);
    const arena_array2 = try arena_allocator.alloc(f32, 5);
    const arena_string = try arena_allocator.dupe(u8, "Arena allocated string");
    
    std.debug.print("アリーナアロケータで確保したメモリ:\n");
    std.debug.print("- 配列1のサイズ: {d}\n", .{arena_array1.len});
    std.debug.print("- 配列2のサイズ: {d}\n", .{arena_array2.len});
    std.debug.print("- 文字列: {s}\n", .{arena_string});
    
    // FixedBufferAllocator
    var buffer: [1024]u8 = undefined;
    var fba = std.heap.FixedBufferAllocator.init(&buffer);
    const fixed_allocator = fba.allocator();
    
    const fixed_array = try fixed_allocator.alloc(i32, 10);
    defer fixed_allocator.free(fixed_array);
    
    for (fixed_array, 0..) |*item, i| {
        item.* = @intCast(i + 1);
    }
    
    std.debug.print("FixedBufferAllocatorで確保した配列: ");
    for (fixed_array) |item| {
        std.debug.print("{d} ", .{item});
    }
    std.debug.print("\n");
    
    // ------------------------------------------
    // メモリリークの検出
    // ------------------------------------------
    
    std.debug.print("\n=== メモリリークの検出 ===\n");
    
    // わざとメモリリークを作成（デモ用）
    demonstrate_memory_management(allocator) catch |err| {
        std.debug.print("メモリ管理エラー: {}\n", .{err});
    };
    
    // ------------------------------------------
    // スマートポインタ的な使用方法
    // ------------------------------------------
    
    std.debug.print("\n=== スマートポインタ的な使用方法 ===\n");
    
    // RAIIパターンを使ったリソース管理
    var resource = try ManagedResource.init(allocator, "重要なデータ");
    defer resource.deinit();
    
    resource.use();
    resource.modify("更新されたデータ");
    resource.use();
}

// ------------------------------------------
// カスタム構造体の定義
// ------------------------------------------

const Person = struct {
    name: []u8,
    age: u32,
    scores: ArrayList(i32),
    
    pub fn deinit(self: *Person, allocator: Allocator) void {
        allocator.free(self.name);
        self.scores.deinit();
    }
    
    pub fn display(self: *const Person) void {
        std.debug.print("名前: {s}, 年齢: {d}\n", .{ self.name, self.age });
        std.debug.print("スコア: ");
        for (self.scores.items) |score| {
            std.debug.print("{d} ", .{score});
        }
        std.debug.print("\n");
    }
    
    pub fn add_score(self: *Person, score: i32) !void {
        try self.scores.append(score);
    }
    
    pub fn average_score(self: *const Person) f32 {
        if (self.scores.items.len == 0) return 0.0;
        
        var sum: i32 = 0;
        for (self.scores.items) |score| {
            sum += score;
        }
        
        return @as(f32, @floatFromInt(sum)) / @as(f32, @floatFromInt(self.scores.items.len));
    }
};

// ------------------------------------------
// メモリ管理のデモンストレーション
// ------------------------------------------

fn demonstrate_memory_management(allocator: Allocator) !void {
    std.debug.print("メモリ管理のデモンストレーション開始\n");
    
    // 正しいメモリ管理の例
    {
        const temp_array = try allocator.alloc(i32, 100);
        defer allocator.free(temp_array);
        
        // 配列を使用
        for (temp_array, 0..) |*item, i| {
            item.* = @intCast(i);
        }
        
        std.debug.print("一時的な配列を作成・使用・解放しました\n");
    }
    
    // ネストしたアロケーションの例
    {
        var nested_list = ArrayList([]i32).init(allocator);
        defer {
            // 内部の配列を解放
            for (nested_list.items) |inner_array| {
                allocator.free(inner_array);
            }
            // 外部のリストを解放
            nested_list.deinit();
        }
        
        // 内部配列を作成
        for (0..3) |i| {
            const inner_array = try allocator.alloc(i32, 5);
            for (inner_array, 0..) |*item, j| {
                item.* = @intCast(i * 5 + j);
            }
            try nested_list.append(inner_array);
        }
        
        std.debug.print("ネストした配列構造を作成・解放しました\n");
    }
}

// ------------------------------------------
// RAIIパターンのデモンストレーション
// ------------------------------------------

const ManagedResource = struct {
    data: []u8,
    allocator: Allocator,
    
    pub fn init(allocator: Allocator, initial_data: []const u8) !ManagedResource {
        const data = try allocator.dupe(u8, initial_data);
        std.debug.print("リソースを初期化しました: {s}\n", .{data});
        
        return ManagedResource{
            .data = data,
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *ManagedResource) void {
        std.debug.print("リソースを解放します: {s}\n", .{self.data});
        self.allocator.free(self.data);
    }
    
    pub fn use(self: *const ManagedResource) void {
        std.debug.print("リソースを使用中: {s}\n", .{self.data});
    }
    
    pub fn modify(self: *ManagedResource, new_data: []const u8) void {
        // 古いデータを解放
        self.allocator.free(self.data);
        
        // 新しいデータをアロケート（エラーハンドリングは簡略化）
        self.data = self.allocator.dupe(u8, new_data) catch {
            std.debug.print("メモリアロケーションに失敗しました\n");
            return;
        };
        
        std.debug.print("リソースを更新しました: {s}\n", .{self.data});
    }
};