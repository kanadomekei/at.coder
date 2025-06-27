# 🚀 クイックスタートガイド

このガイドでは、Rust、Go、Zigの基本文法を素早く学習開始できるよう、最短ルートを案内します。

## ⚡ 5分で始める

### 1. 環境確認

まず、学習したい言語がインストールされているか確認：

```bash
# Rust
rustc --version

# Go  
go version

# Zig
zig version
```

### 2. 最初のプログラムを実行

```bash
cd learning

# Rust
cd rust && rustc 01_variables_types.rs && ./01_variables_types

# Go
cd go && go run 01_variables_types.go

# Zig  
cd zig && zig run 01_variables_types.zig
```

## 📈 学習プラン（言語別）

### 🦀 Rust（推奨期間：2-3週間）

**Week 1: 基礎**
```bash
# Day 1-2: 基本構文
rustc 01_variables_types.rs && ./01_variables_types
rustc 02_control_flow.rs && ./02_control_flow  
rustc 03_functions.rs && ./03_functions

# Day 3-4: 構造体とエラー処理
rustc 04_structs_enums.rs && ./04_structs_enums
rustc 06_error_handling.rs && ./06_error_handling

# Day 5-7: Rustの核心
rustc 05_ownership_lifetimes.rs && ./05_ownership_lifetimes
```

**Week 2: 応用**
```bash
# Day 8-10: 高度な機能
rustc 07_traits_generics.rs && ./07_traits_generics
rustc 08_collections.rs && ./08_collections

# Day 11-14: 実践
rustc 09_practical_examples.rs && ./09_practical_examples
```

### 🐹 Go（推奨期間：1-2週間）

**Week 1: 基礎から実践まで**
```bash
# Day 1-2: 基本構文
go run 01_variables_types.go
go run 02_control_flow.go
go run 03_functions.go

# Day 3-4: 構造体と並行処理
go run 04_structs_interfaces.go
go run 06_goroutines_channels.go

# Day 5-7: エラー処理と実践
go run 05_error_handling.go
go run 07_packages_modules.go
go run 08_practical_examples.go
```

### ⚡ Zig（推奨期間：2-3週間）

**Week 1: 基礎**
```bash
# Day 1-3: 基本構文
zig run 01_variables_types.zig
zig run 02_control_flow.zig
zig run 03_functions.zig

# Day 4-7: 構造体とエラー処理
zig run 04_structs_enums.zig
zig run 05_error_handling.zig
```

**Week 2: Zigの特徴**
```bash
# Day 8-10: メモリ管理
zig run 06_memory_management.zig
zig run 07_arrays_slices.zig

# Day 11-14: 高度な機能
zig run 08_comptime.zig
zig run 09_practical_examples.zig
```

## 🎯 言語選択ガイド

### 初めてのシステムプログラミング
→ **Go** から始める
- 理由: シンプルで分かりやすい
- 次: Rust（安全性を学ぶ）→ Zig（低レベルを学ぶ）

### C/C++から移行
→ **Zig** から始める  
- 理由: C言語に近い感覚
- 次: Rust（現代的な安全性）→ Go（実用性）

### Web開発から移行
→ **Go** から始める
- 理由: Web開発に直結
- 次: Rust（パフォーマンス）→ Zig（システム）

### 学術・研究目的
→ **Rust** から始める
- 理由: 最新の言語設計理論
- 次: Zig（実験的機能）→ Go（実用例）

## 🔥 集中学習（1日コース）

時間が限られている場合の1日集中プラン：

### Goクイック（4時間）
```bash
# 1時間目: 基礎
go run 01_variables_types.go
go run 02_control_flow.go

# 2時間目: 関数と構造体  
go run 03_functions.go
go run 04_structs_interfaces.go

# 3時間目: 並行処理
go run 06_goroutines_channels.go

# 4時間目: 実践
go run 08_practical_examples.go
```

### Rustクイック（6時間）
```bash
# 1-2時間目: 基礎
rustc 01_variables_types.rs && ./01_variables_types
rustc 02_control_flow.rs && ./02_control_flow

# 3-4時間目: 所有権（重要！）
rustc 05_ownership_lifetimes.rs && ./05_ownership_lifetimes

# 5時間目: エラー処理
rustc 06_error_handling.rs && ./06_error_handling

# 6時間目: 実践
rustc 09_practical_examples.rs && ./09_practical_examples
```

## 💡 学習のコツ

### 1. アクティブ学習
```bash
# ❌ ただ読むだけ
cat 01_variables_types.rs

# ✅ 実際に実行して確認
rustc 01_variables_types.rs && ./01_variables_types
```

### 2. 改造実験
```bash
# ✅ サンプルを改造してみる
cp 01_variables_types.rs my_experiment.rs
# my_experiment.rs を編集して実験
rustc my_experiment.rs && ./my_experiment
```

### 3. 比較学習
```bash
# ✅ 同じ概念を3言語で比較
go run go/03_functions.go
rustc rust/03_functions.rs && ./rust/03_functions  
zig run zig/03_functions.zig
```

## 🚨 よくある躓きポイント

### Rust
- **所有権**: 最初は理解しにくいが、05_ownership_lifetimes.rsを繰り返し読む
- **ライフタイム**: 無理に理解しようとせず、まず慣れる

### Go
- **ゴルーチン**: channel操作で詰まりやすい→06_goroutines_channels.goを段階的に理解
- **インターフェース**: duck typingの概念を理解する

### Zig  
- **comptime**: 最初は飛ばしてOK→08_comptime.zigは後回し
- **メモリ管理**: allocator概念が独特→06_memory_management.zigで丁寧に学習

## 📚 次のステップ

学習完了後のロードマップ：

### Rust
1. [The Rust Book](https://doc.rust-lang.org/book/) を読破
2. 小さなCLIツールを作成
3. WebAssemblyプロジェクトに挑戦

### Go
1. Webサーバーを作成
2. gRPCを学習  
3. Kubernetesオペレーターを作成

### Zig
1. C言語ライブラリをZigから呼び出し
2. 組み込みプロジェクトに挑戦
3. Zigコンパイラのコントリビュート

---

**始める準備はできましたか？** `learning/` ディレクトリに移動して、興味のある言語から始めてください！