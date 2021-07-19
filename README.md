# rust-stm32-hal2-template
STM32マイコンをRustで開発するためのテンプレート

## 概要
- Visual Studio Codeでコーディング
- wsl + Rustでビルド
- Visual Studio Code + openOCDでデバッグ

windowsだけで環境を構築したかったが、powershell+Rustだとエラーがでてcortex-m向けのクロスコンパイルができなかったので、緊急避難的にwsl+ubuntuでビルドするようにした。
windows側のエラーが解消されたら切り替える予定。

## 環境構築
### WSL
wsl2の有効化とUbuntu 20.04をインストールしてください
他の組み合わせ(例えばwsl1 + Ubuntu 18.04)とかでは未確認

以下、wslでの環境構築
- apt
    - libssl-dev 
    - build-essential 
    - gcc-arm-none-eabi
- Rust
    - rustup
        - beta or nighty
    - cargo
        - cargo-edit
        - cargo-generate
        - cargo-update

```bash
# gccまわり
$ sudo apt update
$ sudo apt install libssl-dev build-essential gcc-arm-none-eabi

# Rustまわり
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# クロスコンパイル
$ rustup install beta
$ rustup default beta

$ rustup target add thumbv7em-none-eabihf
$ cargo install cargo-edit
$ cargo install cargo-generate 
$ cargo install cargo-update

$ cargo -V
cargo 1.54.0-beta (5ae8d74b3 2021-06-22)
```

### Windows
### chocolatyのインストール
基本的にChocolatyを使って環境を構築していきます。
インストール済みの場合は飛ばしてください。
[Chocolatey Software \| Installing Chocolatey](https://chocolatey.org/install)

- powershellを管理者権限で起動
```powershell
 > Set-ExecutionPolicy Bypass -Scope Process -Force; [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072; iex ((New-Object System.Net.WebClient).DownloadString('https://chocolatey.org/install.ps1'))
```

### chocolatyを使ったインストール
- VScode
- openOCD
- rust
- git
- gcc-arm-embedded
等をインストールします。
自分の環境に合わせて選んでインストールしてください

```powershell
> choco install vscode
> cocho install oepnocd
> cocho install rust
> choco install rustup
> choco install gcc-arm-embedded
```

### windows側のRustの設定
windows側のrustも設定しておく

```powershell
> rustup install nightly-msvc
> rustup default  nightly-msvc

#クロスコンパイル用のツールチェインの追加
> rustup target add thumbv6m-none-eabi thumbv7m-none-eabi thumbv7em-none-eabi thumbv7em-none-eabihf

# cargo周りのインストール
> cargo install cargo-edit
> cargo install cargo-generate
> cargo install cargo-update
```

### Visual Studio Codeの拡張機能
以下の3つを拡張機能からインストール
- Rust
    - 色付け
- rust-analayser
    - 補完機能
    - まだbeta版
- Cortex-Debug
    - デバッグ用

## プロジェクトの作成
適当なワークスペースフォルダを作ってその中でpowershellを起動

```powershell
PS workspace_dir> cargo generate --git https://github.com/Takaaki-MATSUZAWA/rust-stm32-hal2-template

 Project Name : ProjectName
 Renaming project called `ProjectName` to `project-name`...
 Creating project called `project-name`...
[1/9]   Done: .cargo\config
[2/9]   Done: .gitignore
[3/9]   Done: .vscode\launch.json
[4/9]   Done: .vscode\tasks.json
[5/9]   Done: Cargo.lock
[6/9]   Done: Cargo.toml
[7/9]   Done: memory.x
[8/9]   Done: openocd.cfg
[9/9]   Done: src\main.rs
 Done! New project created ...
 
PS workspace_dir> cd ProjectName
# VScodeを起動
PS workspace_dir> code ./
```

## マイコンに合わせて設定を変える


## デバッグ
- マイコンをPCにつなぐ
- VScodeで左の欄から「デバッグと実行」をクリック
- [Cortex Debug]をクリック
- main.rsの適当な場所にブレークポイントを置く
- 実行

## ビルドとbinの書き込み(手動)(ドラッグ&ドロップ)
```bash
$ cargo build
$ arm-none-eabi-objcopy -O binary ./target/thumbv7em-none-eabihf/debug/${PWD##*/} binary.bin

# binary.binを手動でドラッグアンドドロップ
```

## 参考資料
- Rust用のHALライブラリ
[GitHub - David-OConnor/stm32-hal: This library provides access to STM32 peripherals in Rust.](https://github.com/David-OConnor/stm32-hal/tree/main)
- 上記のドキュメント
[stm32_hal2(ドキュメント) - Rust](https://docs.rs/stm32-hal2/0.2.9/stm32_hal2/#)
