# Rust 基本文法ガイド

このディレクトリには、Rustプログラミング言語の基本文法を学ぶためのサンプルコードが含まれています。

## Rustとは

Rustは、システムプログラミング言語として設計された言語です。主な特徴：
- メモリ安全性と高パフォーマンスを両立
- 所有権システムによる自動メモリ管理
- ゼロコスト抽象化
- 並行プログラミングの安全性
- C/C++とのinteroperability

## 学習の進め方

以下の順序でサンプルファイルを実行・確認することをお勧めします：

1. **01_variables_types.rs** - 変数とデータ型
2. **02_control_flow.rs** - 制御構文（if、loop、while、for）
3. **03_functions.rs** - 関数の定義と呼び出し
4. **04_structs_enums.rs** - 構造体とenum
5. **05_ownership_lifetimes.rs** - 所有権とライフタイム
6. **06_error_handling.rs** - エラーハンドリング
7. **07_traits_generics.rs** - トレイトとジェネリクス
8. **08_collections.rs** - コレクション（Vec、HashMap等）
9. **09_practical_examples.rs** - 実践的なサンプル

## ファイルの実行方法

各ファイルは以下のコマンドで実行できます：

```bash
# 実行
rustc filename.rs && ./filename

# または Cargo プロジェクトとして実行
cargo new sample_project
cd sample_project
# src/main.rs にコードをコピー
cargo run

# テスト実行（テストがある場合）
cargo test
```

## 基本的なプログラム構造

Rustプログラムの基本構造：

```rust
fn main() {
    println!("Hello, Rust!");
}
```

## 重要なポイント

- **所有権システム**: メモリ安全性を保証する独自のシステム
- **借用（Borrowing）**: 所有権を移動せずに値を参照する仕組み
- **ライフタイム**: 参照の有効期間を管理
- **パターンマッチング**: `match` 式による強力な分岐処理
- **トレイト**: 型に共通の動作を定義するシステム
- **エラーハンドリング**: `Result<T, E>` と `Option<T>` による安全なエラー処理

## Cargoについて

Rustの標準的なビルドツール・パッケージマネージャ：

```bash
# 新しいプロジェクトを作成
cargo new project_name

# 依存関係をインストール
cargo build

# プログラムを実行
cargo run

# テストを実行
cargo test

# リリースビルド
cargo build --release
```

各サンプルファイルには詳細なコメントが含まれているので、コードを読みながら理解を深めてください。