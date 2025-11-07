# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## プロジェクト概要

`sorde_json-key-order`は、JSONのキー順序に関するRustの実験的なプロジェクトです。
現在は初期段階で、基本的なプロジェクト構造のみが存在しています。

## 開発コマンド

### ビルドと実行
```bash
cargo build              # ビルド
cargo build --release    # リリースビルド
cargo run                # 実行
```

### テストとコード品質
```bash
cargo test               # テスト実行
cargo test <test_name>   # 特定のテストを実行
cargo check              # 型チェックのみ実行（高速）
cargo clippy             # リント実行
cargo fmt                # コードフォーマット
cargo fmt -- --check     # フォーマットチェックのみ
```

## プロジェクト構造

- **Rust Edition**: 2024（最新エディション）
- **エントリポイント**: `src/main.rs`
- **依存関係**: 現在は依存関係なし

## 実装時の考慮事項

プロジェクト名から、このプロジェクトはserde_jsonでのJSONキー順序の保持に関する機能を実装する予定と思われます。実装時には以下の依存関係の追加を検討してください：

- `serde` + `serde_json`: JSON処理の基本
- `indexmap`: 挿入順序を保持するHashMap実装
- `serde_json::Map<String, Value>`: デフォルトではBTreeMapを使用（キーがソートされる）

キー順序の保持方法：
1. `preserve_order`フィーチャーを有効化する方法
2. カスタムデシリアライザを実装する方法
3. `indexmap`を直接使用する方法
