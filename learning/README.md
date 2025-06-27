# プログラミング言語学習ガイド

このディレクトリには、Rust、Go、Zigの3つのシステムプログラミング言語の基本文法を学ぶためのサンプルコードが含まれています。

## 📁 ディレクトリ構造

```
learning/
├── README.md          # このファイル（統合学習ガイド）
├── rust/              # Rust学習教材
│   ├── README.md      # Rust専用学習ガイド
│   ├── 01_variables_types.rs
│   ├── 02_control_flow.rs
│   ├── 03_functions.rs
│   ├── 04_structs_enums.rs
│   ├── 05_ownership_lifetimes.rs
│   ├── 06_error_handling.rs
│   ├── 07_traits_generics.rs
│   ├── 08_collections.rs
│   └── 09_practical_examples.rs
├── go/                # Go学習教材
│   ├── README.md      # Go専用学習ガイド
│   ├── 01_variables_types.go
│   ├── 02_control_flow.go
│   ├── 03_functions.go
│   ├── 04_structs_interfaces.go
│   ├── 05_error_handling.go
│   ├── 06_goroutines_channels.go
│   ├── 07_packages_modules.go
│   └── 08_practical_examples.go
└── zig/               # Zig学習教材
    ├── README.md      # Zig専用学習ガイド
    ├── 01_variables_types.zig
    ├── 02_control_flow.zig
    ├── 03_functions.zig
    ├── 04_structs_enums.zig
    ├── 05_error_handling.zig
    ├── 06_memory_management.zig
    ├── 07_arrays_slices.zig
    ├── 08_comptime.zig
    └── 09_practical_examples.zig
```

## 🎯 学習の進め方

### 推奨学習順序

1. **どれか1つの言語を選んで基礎を学ぶ**
   - 初心者には **Go** がおすすめ（シンプルで理解しやすい）
   - 経験者には **Rust** がおすすめ（現代的な安全性機能）
   - 低レベル志向なら **Zig** がおすすめ（C言語の代替）

2. **各言語のファイルを順番に実行・理解**
   - `01_variables_types.*` から始める
   - 各ファイルのコメントを読みながら理解
   - 実際にコードを実行して動作確認

3. **言語比較学習**
   - 同じ概念（例：エラーハンドリング）を3言語で比較
   - 各言語の特徴や哲学の違いを理解

## 🔧 実行方法

### Rust
```bash
cd learning/rust
rustc 01_variables_types.rs && ./01_variables_types
# または
cargo new sample_project
cd sample_project
# src/main.rs にコードをコピーして
cargo run
```

### Go
```bash
cd learning/go
go run 01_variables_types.go
```

### Zig
```bash
cd learning/zig
zig run 01_variables_types.zig
```

## 📊 言語比較表

| 特徴 | Rust | Go | Zig |
|------|------|----|----|
| **パフォーマンス** | 🟢 高速 | 🟡 高速 | 🟢 高速 |
| **メモリ安全性** | 🟢 所有権システム | 🟡 GC | 🟢 明示的管理 |
| **学習コスト** | 🔴 高い | 🟢 低い | 🟡 中程度 |
| **並行処理** | 🟡 async/await | 🟢 ゴルーチン | 🟡 基本的サポート |
| **エラーハンドリング** | 🟢 Result型 | 🟡 error値 | 🟢 error union |
| **C互換性** | 🟡 FFI | 🟡 cgo | 🟢 直接的 |
| **コンパイル速度** | 🔴 遅い | 🟢 速い | 🟢 速い |
| **エコシステム** | 🟢 豊富 | 🟢 豊富 | 🔴 発展中 |

## 🚀 各言語の特徴と適用場面

### Rust 🦀
**特徴:**
- 所有権システムによるメモリ安全性
- ゼロコスト抽象化
- 強力な型システム
- パターンマッチング

**適用場面:**
- システムプログラミング
- WebAssembly
- ブロックチェーン
- 組み込みシステム

### Go 🐹
**特徴:**
- シンプルで読みやすい構文
- ゴルーチンによる軽量並行処理
- 高速なコンパイル
- 豊富な標準ライブラリ

**適用場面:**
- Webサービス
- マイクロサービス
- ネットワークプログラミング
- DevOpsツール

### Zig ⚡
**特徴:**
- コンパイル時計算（comptime）
- 明示的なメモリ管理
- C言語との高い互換性
- ランタイムのない設計

**適用場面:**
- C言語の代替
- 組み込みシステム
- ゲーム開発
- システムプログラミング

## 📚 学習トピック比較

### 1. 変数とデータ型
- **Rust**: 不変性がデフォルト、シャドーイング
- **Go**: 型推論（`:=`）、ゼロ値
- **Zig**: コンパイル時型推論、型強制

### 2. 制御構文
- **Rust**: パターンマッチング（`match`）
- **Go**: シンプルな`for`、`switch`
- **Zig**: コンパイル時実行、`defer`

### 3. 関数
- **Rust**: クロージャ、所有権を考慮した引数
- **Go**: 複数返り値、レシーバー
- **Zig**: ジェネリック、コンパイル時関数

### 4. エラーハンドリング
- **Rust**: `Result<T, E>`、`?`演算子
- **Go**: `error`インターフェース、明示的チェック
- **Zig**: `error union`、`try`文

### 5. メモリ管理
- **Rust**: 所有権、借用、ライフタイム
- **Go**: ガベージコレクション
- **Zig**: 明示的アロケータ、手動管理

## 🎓 学習のヒント

1. **実際にコードを書く**: サンプルを見るだけでなく、自分で変更してみる
2. **エラーメッセージを読む**: 各言語のコンパイラのエラーメッセージから学ぶ
3. **コミュニティを活用**: 各言語の公式ドキュメントやフォーラムを参照
4. **小さなプロジェクトを作る**: 学んだ概念を使って実際のプログラムを作成
5. **比較しながら学ぶ**: 同じ機能を3言語で実装して違いを理解

## 📖 追加リソース

### Rust
- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)

### Go
- [A Tour of Go](https://tour.golang.org/)
- [Effective Go](https://golang.org/doc/effective_go.html)

### Zig
- [Zig Language Reference](https://ziglang.org/documentation/master/)
- [Zig Learn](https://ziglearn.org/)

## 🤝 学習サポート

各言語のディレクトリには詳細な`README.md`があります。分からないことがあれば：

1. まず該当言語の`README.md`を確認
2. コード内のコメントを詳しく読む
3. 公式ドキュメントを参照
4. コミュニティフォーラムで質問

Happy Learning! 🎉