# domain

## 概要

ccc の中核となる型を定義する crate です。

AST・計算値・静的型・エラー・時刻のバリューオブジェクトを提供します。

他の crate に依存しません。
依存グラフの最下層に位置します。

## 背景

電卓のロジックは「式の構造」と「値の意味」に強く依存します。

これらの定義が各レイヤーに散らばると、
仕様変更のたびに全体を追う必要が生じます。

そこで、意味の定義をこの crate に集約しています。

## 目的

- 式・値・型・エラーの定義を1箇所にまとめる
- 不正な状態を型で表現できなくする
- パーサーや評価器の実装詳細から、意味の定義を切り離す

## 手法

### インターフェース定義 (依存性逆転)

`interface/` に `CccParser` / `CccTypeChecker` / `CccEvaluator` を定義します。

実装は infrastructure crate が提供します。
上位レイヤーはこの trait にのみ依存します。

### バリューオブジェクト

`time/` に3つの newtype を定義します。

- `DurationSeconds` — 符号付きの時間幅
- `EpochSeconds` — chrono で表現可能なことを保証する時点
- `UtcOffset` — 表示可能な範囲(±24h)を保証するタイムゾーンオフセット

検証はコンストラクタで行います。
構築できた値は、以降の検証なしで安全に使えます。

### 主要モジュール

| モジュール | 内容 |
|---|---|
| `ast` | 式の構造 (`Expression`, `BinaryOperation` など) |
| `value` | 計算結果 (`Value`) と表示 (`value_display`) |
| `static_type` | 型検査で使う静的型 (`StaticType`) |
| `time` | 時刻のバリューオブジェクト |
| `error` | エラー型 (`CccError`) と発生位置 |
| `calendar` | 暦のユーティリティ |
| `interface` | パーサー・型検査器・評価器の trait |

## 処理の事例

`1 + 2 * 3` という式は、次の AST で表現されます。

```rust
use domain::ast::{BinaryOperation, Expression};

Expression::BinaryOperation {
    operator: BinaryOperation::Add,
    left: Box::new(Expression::Integer(1)),
    right: Box::new(Expression::BinaryOperation {
        operator: BinaryOperation::Multiply,
        left: Box::new(Expression::Integer(2)),
        right: Box::new(Expression::Integer(3)),
    }),
}
```

時刻のバリューオブジェクトは、範囲外の値を構築段階で拒否します。

```rust
use domain::time::UtcOffset;

let tokyo = UtcOffset::from_seconds(9 * 3600); // Some(UtcOffset)
let invalid = UtcOffset::from_seconds(30 * 3600); // None (±24h を超える)
```
