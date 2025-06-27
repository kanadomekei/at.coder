#!/bin/bash -eu

# 一時ディレクトリにプロジェクトを作成し、中身をカレントディレクトリに移動する
TMP_DIR="__tmp_cargo_proj"
cargo new "$TMP_DIR" --bin
# 通常のファイルを移動
mv -n "$TMP_DIR"/* .
# 隠しファイルを安全に移動 (. と .. を除く)
find "$TMP_DIR" -maxdepth 1 -name ".?*" -print0 | xargs -0 -I {} mv -n {} .
rm -rf "$TMP_DIR"

# 便利なクレートを依存関係に追加
cargo add proconio@0.3.6
cargo add itertools@0.9.0

# 実行後、このスクリプト自体を削除する
rm -- "$0" 