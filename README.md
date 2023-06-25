# Arduino 互換の `Elegoo Uno R3` で遊ぶリポジトリ

## 使い方

### C言語

1. USBシリアルケーブルをつなぐ(ドライバのインストールとかは不要だった)
2. Arduino IDEを開く
3. ツール > ボード > Arduino AVR Boards > Arduino Uno を選択
4. ツールバー下の右矢印ボタンを押下して、マイコンボードに書き込む

### Rust

1. [arv-hal](https://github.com/Rahix/avr-hal)をインストール
    ※Quickstart参照
2. テンプレートからプロジェクト作成

```
cargo install cargo-generate
cargo generate --git https://github.com/Rahix/avr-hal-template.git
```
3. `cargo run`でコンパイル+書き込みされる

