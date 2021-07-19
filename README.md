# rust-stm32-hal2-template
STM32マイコンをRustで開発するためのテンプレート（windows用）

## 概要
- Rustでコーディング
- cargoでクロスコンパイル
- openOCDでデバッグ
- 上記の全てをVScode上で行う

## デモ
![demo](https://github.com/Takaaki-MATSUZAWA/rust-stm32-hal2-template-demo-video/raw/main/debug_test.gif)

## 環境構築
### chocolatyのインストール
基本的にChocolatyを使って環境を構築する  
インストール済みの場合は飛ばしてください  
[Chocolatey Software \| Installing Chocolatey](https://chocolatey.org/install)

- powershellを管理者権限で起動
```powershell
 > Set-ExecutionPolicy Bypass -Scope Process -Force; [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072; iex ((New-Object System.Net.WebClient).DownloadString('https://chocolatey.org/install.ps1'))
```

### パッケージのインストール
インストール済みのものは飛ばしてインストールしてください

```powershell
> choco install vscode
> choco install git
> choco install mingw
> cocho install oepnocd
> cocho install rust
> choco install rustup
> choco install gcc-arm-embedded
```

### Rustの環境構築
クロスコンパイル用の環境構築

```powershell
> rustup install nightly-msvc
> rustup default nightly-msvc

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
適当な場所にワークスペースフォルダを作ってその中でpowershellを起動

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
以下の例はstm32g431をターゲットに設定されています  
ターゲットマイコンに合わせて設定を書き換えてください

### .cargo/config.toml
ターゲットマイコンの種類に合わせて1つだけコメントを外す
```config
...
# target = "thumbv6m-none-eabi"        # Cortex-M0 and Cortex-M0+
# target = "thumbv7m-none-eabi"        # Cortex-M3
# target = "thumbv7em-none-eabi"       # Cortex-M4 and Cortex-M7 (no FPU)
target = "thumbv7em-none-eabihf"     # Cortex-M4F and Cortex-M7F (with FPU)
...
```

### .vscode/launch.json
デバッグ用の設定  
マイコンに合わせて"device"と".cfg"ファイルを変更
```json
...
"device": "STM32G431",
"configFiles": [
    "interface/stlink-v2-1.cfg",
    "target/stm32g4x.cfg"
],
...
```

### Cargo.toml
こちらもマイコンに合わせてfeaturesを編集
```config
...
stm32-hal2 = { version = "^0.2.9", features = ["g431", "g4rt"]}
...
```

### memory.x
マイコンのFLASHとRAMの容量、アドレスにあせて変更してください  
FLASH領域の一部をユーザーデータの保存等に使うときも適宜書き換えてください
```x
...
FLASH : ORIGIN = 0x08000000, LENGTH = 64K
RAM : ORIGIN = 0x20000000, LENGTH = 12K
...
```

## デバッグ
- マイコンをPCにつなぐ
- VScodeで左の欄から「デバッグと実行」をクリック
- [Cortex Debug]をクリック
- main.rsの適当な場所にブレークポイントを置く
- 実行

## ビルドとbinの書き込み(手動)
```bash
$ ${env:HOMEPATH}\\.rustup\\toolchains\\nightly-x86_64-pc-windows-msvc\\bin\\cargo.exe build
$ arm-none-eabi-objcopy -O binary ./target/thumbv7em-none-eabihf/debug/${PWD##*/} binary.bin
```
出来上がったbinary.binを手動で書き込み  
nucleo等ならドラッグアンドドロップ

## 参考資料
- Rust用のHALライブラリ
    - [GitHub - David-OConnor/stm32-hal: This library provides access to STM32 peripherals in Rust.](https://github.com/David-OConnor/stm32-hal/tree/main)
- 上記のドキュメント
    - [stm32_hal2(ドキュメント) - Rust](https://docs.rs/stm32-hal2/0.2.9/stm32_hal2/#)
