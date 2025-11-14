# lsb_hide

[![Rust CI & Release](https://github.com/romeoahmed/lsb_hide/actions/workflows/rust.yml/badge.svg)](https://github.com/romeoahmed/lsb_hide/actions/workflows/rust.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

一款基于 LSB (最低有效位) 隐写术的命令行工具，用于在 24 位 BMP 图像中安全地隐藏或恢复文本文件。

## ✨ 功能

- **隐藏文本**: 将任意文本文件的内容嵌入到 BMP 图像的像素数据中。
- **恢复文本**: 从已嵌入信息的 BMP 图像中提取并恢复原始文本文件。
- **跨平台**: 支持在 Windows 和 Linux 上编译和运行。
- **安全可靠**: 在执行操作前会进行空间检查，并提供清晰的错误提示。
- **简单易用**: 提供简洁的命令行接口。

## 📦 安装

你可以通过以下两种方式安装 `lsb_hide`：

### 1. 从 GitHub Releases 下载 (推荐)

对于 Windows 和 Linux 用户，最简单的方式是从项目的 [Releases 页面](https://github.com/romeoahmed/lsb_hide/releases) 下载最新的预编译二进制文件。

1.  前往最新的 Release 页面。
2.  下载对应你操作系统的压缩包（`.zip` for Windows, `.tar.zst` for Linux）。
3.  解压后即可直接在命令行中使用。

### 2. 使用 `cargo` 从源码安装

如果你已经安装了 [Rust 工具链](https://www.rust-lang.org/tools/install)，你可以直接从源码安装：

```bash
cargo install --git https://github.com/romeoahmed/lsb_hide.git
```

## 🚀 使用方法

`lsb_hide` 主要包含两个子命令：`hide` 和 `recover`。

### 隐藏文本

使用 `hide` 命令将文本文件隐藏到 BMP 图像中。

```bash
lsb_hide hide --image <原始图像.bmp> --text <要隐藏的文本.txt> --destination <输出图像.bmp>
```

**参数说明:**

- `-i, --image`: 原始的 BMP 图像文件路径。
- `-t, --text`: 要隐藏的文本文件路径。
- `-d, --destination`: 嵌入信息后要保存的新图像文件路径。

**示例:**

```bash
lsb_hide hide -i input.bmp -t secret.txt -d output.bmp
```

### 恢复文本

使用 `recover` 命令从图像中恢复隐藏的文本。

```bash
lsb_hide recover --image <已嵌入信息的图像.bmp> --text <恢复后的文本文件.txt>
```

**参数说明:**

- `-i, --image`: 包含隐藏信息的 BMP 图像文件路径。
- `-t, --text`: 恢复出的文本要保存到的文件路径。

**示例:**

```bash
lsb_hide recover -i output.bmp -t recovered.txt
```

## 🛠️ 从源码构建

如果你想自己编译项目：

1.  克隆仓库：
    ```bash
    git clone https://github.com/romeoahmed/lsb_hide.git
    cd lsb_hide
    ```

2.  构建项目 (Release 模式):
    ```bash
    cargo build --release
    ```

3.  编译后的可执行文件位于 `./target/release/lsb_hide`。

## 📄 许可证

本项目采用 [MIT 许可证](LICENSE)。
