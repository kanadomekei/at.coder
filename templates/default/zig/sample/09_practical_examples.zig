const std = @import("std");
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Allocator = std.mem.Allocator;

pub fn main() !void {
    // ==========================================
    // 実践的なサンプル
    // ==========================================
    
    std.debug.print("=== Zigの実践的なサンプル ===\n");
    
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    // ------------------------------------------
    // 簡単な計算機
    // ------------------------------------------
    
    std.debug.print("\n=== 簡単な計算機 ===\n");
    
    var calculator = Calculator.init();
    
    const expr1 = "10 + 5 * 2";
    const result1 = calculator.evaluate(expr1) catch |err| {
        std.debug.print("計算エラー: {}\n", .{err});
        0;
    };
    std.debug.print("{s} = {d}\n", .{ expr1, result1 });
    
    const expr2 = "100 / (2 + 3)";
    const result2 = calculator.evaluate(expr2) catch |err| {
        std.debug.print("計算エラー: {}\n", .{err});
        0;
    };
    std.debug.print("{s} = {d}\n", .{ expr2, result2 });
    
    // ------------------------------------------
    // 学生成績管理システム
    // ------------------------------------------
    
    std.debug.print("\n=== 学生成績管理システム ===\n");
    
    var grade_manager = GradeManager.init(allocator);
    defer grade_manager.deinit();
    
    // 学生の追加
    try grade_manager.add_student("Alice", 1001);
    try grade_manager.add_student("Bob", 1002);
    try grade_manager.add_student("Charlie", 1003);
    
    // 成績の追加
    try grade_manager.add_grade(1001, "数学", 85);
    try grade_manager.add_grade(1001, "英語", 92);
    try grade_manager.add_grade(1001, "理科", 78);
    
    try grade_manager.add_grade(1002, "数学", 90);
    try grade_manager.add_grade(1002, "英語", 88);
    
    try grade_manager.add_grade(1003, "数学", 75);
    try grade_manager.add_grade(1003, "英語", 95);
    try grade_manager.add_grade(1003, "理科", 82);
    
    // 成績の表示
    grade_manager.display_all_students();
    
    // 平均点の計算
    const alice_avg = grade_manager.get_student_average(1001) catch 0.0;
    std.debug.print("Aliceの平均点: {d:.2}\n", .{alice_avg});
    
    // ------------------------------------------
    // ファイル処理システム
    // ------------------------------------------
    
    std.debug.print("\n=== ファイル処理システム ===\n");
    
    var file_processor = FileProcessor.init(allocator);
    defer file_processor.deinit();
    
    // テキストファイルの処理
    try file_processor.process_text("sample.txt");
    
    // ------------------------------------------
    // データ構造：二分探索木
    // ------------------------------------------
    
    std.debug.print("\n=== 二分探索木 ===\n");
    
    var bst = BinarySearchTree(i32).init(allocator);
    defer bst.deinit();
    
    // ノードの挿入
    const values = [_]i32{ 50, 30, 70, 20, 40, 60, 80 };
    for (values) |value| {
        try bst.insert(value);
    }
    
    std.debug.print("挿入した値: ");
    for (values) |value| {
        std.debug.print("{d} ", .{value});
    }
    std.debug.print("\n");
    
    // 中順巡回
    std.debug.print("中順巡回: ");
    bst.inorder_traversal();
    std.debug.print("\n");
    
    // 値の検索
    const search_values = [_]i32{ 40, 45, 80 };
    for (search_values) |value| {
        if (bst.search(value)) {
            std.debug.print("{d} が見つかりました\n", .{value});
        } else {
            std.debug.print("{d} は見つかりませんでした\n", .{value});
        }
    }
    
    // ------------------------------------------
    // ネットワーク通信シミュレーション
    // ------------------------------------------
    
    std.debug.print("\n=== ネットワーク通信シミュレーション ===\n");
    
    var network_sim = NetworkSimulator.init(allocator);
    defer network_sim.deinit();
    
    // デバイスの追加
    try network_sim.add_device("Router", "192.168.1.1");
    try network_sim.add_device("PC1", "192.168.1.100");
    try network_sim.add_device("PC2", "192.168.1.101");
    try network_sim.add_device("Server", "192.168.1.10");
    
    // パケットの送信
    try network_sim.send_packet("PC1", "Server", "HTTP Request");
    try network_sim.send_packet("Server", "PC1", "HTTP Response");
    try network_sim.send_packet("PC2", "Router", "DNS Query");
    
    // ネットワーク状態の表示
    network_sim.display_devices();
    network_sim.display_packet_log();
}

// ------------------------------------------
// 計算機クラス
// ------------------------------------------

const Calculator = struct {
    pub fn init() Calculator {
        return Calculator{};
    }
    
    pub fn evaluate(self: *Calculator, expression: []const u8) !i32 {
        _ = self; // self は使用されないが、メソッド形式のため
        
        // 簡単な式の評価（実際の実装ではパーサーを使用）
        if (std.mem.eql(u8, expression, "10 + 5 * 2")) {
            return 20; // 10 + (5 * 2)
        } else if (std.mem.eql(u8, expression, "100 / (2 + 3)")) {
            return 20; // 100 / 5
        } else {
            return error.UnsupportedExpression;
        }
    }
};

// ------------------------------------------
// 学生成績管理システム
// ------------------------------------------

const Student = struct {
    name: []const u8,
    id: u32,
    grades: HashMap([]const u8, i32, StringContext, std.hash_map.default_max_load_percentage),
    
    const StringContext = struct {
        pub fn hash(self: @This(), s: []const u8) u64 {
            _ = self;
            return std.hash_map.hashString(s);
        }
        
        pub fn eql(self: @This(), a: []const u8, b: []const u8) bool {
            _ = self;
            return std.mem.eql(u8, a, b);
        }
    };
    
    pub fn init(allocator: Allocator, name: []const u8, id: u32) Student {
        return Student{
            .name = name,
            .id = id,
            .grades = HashMap([]const u8, i32, StringContext, std.hash_map.default_max_load_percentage).init(allocator),
        };
    }
    
    pub fn deinit(self: *Student) void {
        self.grades.deinit();
    }
    
    pub fn add_grade(self: *Student, subject: []const u8, grade: i32) !void {
        try self.grades.put(subject, grade);
    }
    
    pub fn get_average(self: *const Student) f32 {
        if (self.grades.count() == 0) return 0.0;
        
        var total: i32 = 0;
        var iterator = self.grades.iterator();
        while (iterator.next()) |entry| {
            total += entry.value_ptr.*;
        }
        
        return @as(f32, @floatFromInt(total)) / @as(f32, @floatFromInt(self.grades.count()));
    }
    
    pub fn display(self: *const Student) void {
        std.debug.print("学生: {s} (ID: {d})\n", .{ self.name, self.id });
        
        var iterator = self.grades.iterator();
        while (iterator.next()) |entry| {
            std.debug.print("  {s}: {d}点\n", .{ entry.key_ptr.*, entry.value_ptr.* });
        }
        
        const avg = self.get_average();
        std.debug.print("  平均点: {d:.2}\n", .{avg});
    }
};

const GradeManager = struct {
    students: ArrayList(Student),
    allocator: Allocator,
    
    pub fn init(allocator: Allocator) GradeManager {
        return GradeManager{
            .students = ArrayList(Student).init(allocator),
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *GradeManager) void {
        for (self.students.items) |*student| {
            student.deinit();
        }
        self.students.deinit();
    }
    
    pub fn add_student(self: *GradeManager, name: []const u8, id: u32) !void {
        const student = Student.init(self.allocator, name, id);
        try self.students.append(student);
    }
    
    pub fn find_student(self: *GradeManager, id: u32) ?*Student {
        for (self.students.items) |*student| {
            if (student.id == id) {
                return student;
            }
        }
        return null;
    }
    
    pub fn add_grade(self: *GradeManager, student_id: u32, subject: []const u8, grade: i32) !void {
        if (self.find_student(student_id)) |student| {
            try student.add_grade(subject, grade);
        } else {
            return error.StudentNotFound;
        }
    }
    
    pub fn get_student_average(self: *GradeManager, student_id: u32) !f32 {
        if (self.find_student(student_id)) |student| {
            return student.get_average();
        } else {
            return error.StudentNotFound;
        }
    }
    
    pub fn display_all_students(self: *const GradeManager) void {
        std.debug.print("\n=== 全学生の成績 ===\n");
        for (self.students.items) |*student| {
            student.display();
            std.debug.print("\n");
        }
    }
};

// ------------------------------------------
// ファイル処理システム
// ------------------------------------------

const FileProcessor = struct {
    allocator: Allocator,
    
    pub fn init(allocator: Allocator) FileProcessor {
        return FileProcessor{
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *FileProcessor) void {
        _ = self; // 特にクリーンアップする必要がない
    }
    
    pub fn process_text(self: *FileProcessor, filename: []const u8) !void {
        _ = self;
        
        // ファイル処理のシミュレーション
        std.debug.print("ファイル '{s}' を処理中...\n", .{filename});
        
        // 仮のテキスト内容
        const content = 
            \\This is a sample text file.
            \\It contains multiple lines.
            \\Each line will be processed.
            \\The end of the file.
        ;
        
        var line_count: u32 = 0;
        var word_count: u32 = 0;
        var char_count: u32 = 0;
        
        var lines = std.mem.split(u8, content, "\n");
        while (lines.next()) |line| {
            line_count += 1;
            char_count += @intCast(line.len);
            
            var words = std.mem.split(u8, line, " ");
            while (words.next()) |word| {
                if (word.len > 0) {
                    word_count += 1;
                }
            }
        }
        
        std.debug.print("ファイル統計:\n");
        std.debug.print("  行数: {d}\n", .{line_count});
        std.debug.print("  単語数: {d}\n", .{word_count});
        std.debug.print("  文字数: {d}\n", .{char_count});
    }
};

// ------------------------------------------
// 二分探索木
// ------------------------------------------

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
        
        pub fn deinit(self: *Self) void {
            self.destroy_node(self.root);
        }
        
        fn destroy_node(self: *Self, node: ?*Node) void {
            if (node) |n| {
                self.destroy_node(n.left);
                self.destroy_node(n.right);
                self.allocator.destroy(n);
            }
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
            _ = self;
            if (node == null) return false;
            
            if (value == node.?.value) return true;
            if (value < node.?.value) return self.search_node(node.?.left, value);
            return self.search_node(node.?.right, value);
        }
        
        pub fn inorder_traversal(self: *const Self) void {
            self.inorder_node(self.root);
        }
        
        fn inorder_node(self: *const Self, node: ?*Node) void {
            if (node) |n| {
                self.inorder_node(n.left);
                std.debug.print("{d} ", .{n.value});
                self.inorder_node(n.right);
            }
        }
    };
}

// ------------------------------------------
// ネットワーク通信シミュレーション
// ------------------------------------------

const NetworkDevice = struct {
    name: []const u8,
    ip_address: []const u8,
};

const Packet = struct {
    from: []const u8,
    to: []const u8,
    data: []const u8,
    timestamp: u64,
};

const NetworkSimulator = struct {
    devices: ArrayList(NetworkDevice),
    packet_log: ArrayList(Packet),
    allocator: Allocator,
    
    pub fn init(allocator: Allocator) NetworkSimulator {
        return NetworkSimulator{
            .devices = ArrayList(NetworkDevice).init(allocator),
            .packet_log = ArrayList(Packet).init(allocator),
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *NetworkSimulator) void {
        self.devices.deinit();
        self.packet_log.deinit();
    }
    
    pub fn add_device(self: *NetworkSimulator, name: []const u8, ip: []const u8) !void {
        const device = NetworkDevice{
            .name = name,
            .ip_address = ip,
        };
        try self.devices.append(device);
    }
    
    pub fn send_packet(self: *NetworkSimulator, from: []const u8, to: []const u8, data: []const u8) !void {
        const packet = Packet{
            .from = from,
            .to = to,
            .data = data,
            .timestamp = std.time.milliTimestamp(),
        };
        try self.packet_log.append(packet);
    }
    
    pub fn display_devices(self: *const NetworkSimulator) void {
        std.debug.print("\n=== ネットワークデバイス ===\n");
        for (self.devices.items) |device| {
            std.debug.print("{s}: {s}\n", .{ device.name, device.ip_address });
        }
    }
    
    pub fn display_packet_log(self: *const NetworkSimulator) void {
        std.debug.print("\n=== パケットログ ===\n");
        for (self.packet_log.items) |packet| {
            std.debug.print("{s} -> {s}: {s}\n", .{ packet.from, packet.to, packet.data });
        }
    }
};