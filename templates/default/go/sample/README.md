# Go 基本文法ガイド

このディレクトリには、Goプログラミング言語の基本文法を学ぶためのサンプルコードが含まれています。

## Goとは

Goは、Googleによって開発されたオープンソースのプログラミング言語です。主な特徴：
- シンプルで読みやすい構文
- 高速なコンパイル
- 強力な並行処理サポート（ゴルーチンとチャネル）
- 静的型付け
- ガベージコレクション
- 豊富な標準ライブラリ

## 学習の進め方

以下の順序でサンプルファイルを実行・確認することをお勧めします：

1. **01_variables_types.go** - 変数とデータ型
2. **02_control_flow.go** - 制御構文（if、for、switch）
3. **03_functions.go** - 関数の定義と呼び出し
4. **04_structs_interfaces.go** - 構造体とインターフェース
5. **05_error_handling.go** - エラーハンドリング
6. **06_goroutines_channels.go** - ゴルーチンとチャネル
7. **07_packages_modules.go** - パッケージとモジュール
8. **08_practical_examples.go** - 実践的なサンプル

## ファイルの実行方法

各ファイルは以下のコマンドで実行できます：

```bash
# 直接実行
go run filename.go

# コンパイルしてから実行
go build filename.go
./filename

# テスト実行（テストファイルがある場合）
go test

# モジュールの初期化（プロジェクトルートで）
go mod init module-name

# 依存関係の管理
go mod tidy
```

## 基本的なプログラム構造

Goプログラムの基本構造：

```go
package main

import "fmt"

func main() {
    fmt.Println("Hello, Go!")
}
```

## 重要なポイント

- **パッケージシステム**: すべてのGoコードはパッケージに属する
- **ゴルーチン**: 軽量なスレッドによる並行処理
- **チャネル**: ゴルーチン間の安全な通信手段
- **インターフェース**: ダックタイピングによる型安全性
- **エラーハンドリング**: 明示的なエラー値による処理
- **ポインタ**: メモリアドレスの直接操作（ポインタ演算はなし）

## Go Modulesについて

Goの依存関係管理システム：

```bash
# 新しいモジュールを作成
go mod init example.com/myproject

# 依存関係を追加
go get github.com/some/package

# 依存関係をクリーンアップ
go mod tidy

# 依存関係の確認
go list -m all

# ベンダリング
go mod vendor
```

## 開発ツール

Goには優秀な開発ツールが含まれています：

```bash
# コードのフォーマット
go fmt

# コードの静的解析
go vet

# ドキュメントの生成・表示
go doc

# ベンチマークテスト
go test -bench=.

# プロファイリング
go test -cpuprofile=cpu.prof
```

各サンプルファイルには詳細なコメントが含まれているので、コードを読みながら理解を深めてください。