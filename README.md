# Loto Random CLI

ロト6・ロト7の番号を選ぶ、遊びと乱数教材のための Rust CLI です。

## インストール

[GitHub Releases](https://github.com/mt4110/loto_cli/releases) から macOS arm64 または Linux x86-64 用のアーカイブを取得し、展開したバイナリを実行します。ソースから導入する場合は、リポジトリ内で次を実行します。

```bash
cargo install --locked --path .
loto_random_cli --version
```

Rustup、mise、Nix の開発環境は Rust 1.100.0 を使用します。Nix の場合は先に `nix develop` を実行します。

## 使い方

```bash
# OSの乱数で均等に選ぶ
loto_random_cli loto6 pure --n 3 --out tickets.csv

# 占いを組み合わせて選ぶ
loto_random_cli loto7 oracle --birth-date 1990-05-17 --birth-hour 8 --aura-color gold --n 3

# 1等の確率と毎週1口買う場合の平均待ち時間
loto_random_cli loto6 odds
```

神託モードは、月相の概算、簡易四柱推命、十二支と色を使う東洋占い、運命数、太陽星座を重ねて数字に重みを付けます。当せん確率は変わりません。節入りは固定日、月相は平均周期で概算し、日柱は午前0時で切り替えます。出生時を省略すると時柱を省き、生年月日を省略すると日本時間の今日を参照します。入力はローカルで処理します。

対話端末の神託モードでは Enter を押すまでの時間も数字選びに反映します。自動実行では Enter 待ちを省きます。対話端末でも `--no-prompt` で省けます。`divine` と `destiny` は `oracle` の別名です。旧来の `spread`、`cluster`、`favorite` は引き続き `pure` と同じ処理です。

ロト6の1等は1口あたり **1 / 6,096,454**。毎週1口（年52口）、独立な抽せんと仮定すると、初当せんまでの平均は約 **117,240年** です。出典は[宝くじ公式サイト](https://www.takarakuji-official.jp/brand/suji/lineup-loto6/)です。

## 乱数教材

```bash
# 同じコマンドを2回実行すると、同じ番号列が出る
loto_random_cli loto6 pure --rng seeded --seed 42 --n 3
loto_random_cli loto6 pure --rng seeded --seed 42 --n 3

# getrandom で OS の乱数を取得する既定モード
loto_random_cli loto6 pure --rng os --n 3
```

教材モードは予測可能な32ビットの線形合同法です。秘密情報やトークンの生成には使えません。JavaScript の `Math.random()` が代表する非暗号用途の疑似乱数と OS の乱数の違いを、シードによる再現で体験できます。

## リリースの検証

GitHub Releases のアーカイブは GitHub Actions のビルド証明付きです。`<archive>` をファイル名に置き換えて検証できます。

```bash
gh attestation verify <archive> --repo mt4110/loto_cli
```

脆弱性の連絡方法は [SECURITY.md](SECURITY.md) を参照してください。
