# lsb_hide

[![Rust CI & Release](https://github.com/romeoahmed/lsb_hide/actions/workflows/rust.yaml/badge.svg)](https://github.com/romeoahmed/lsb_hide/actions/workflows/rust.yaml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

一款基于 LSB (最低有效位) 隐写术的命令行工具，用于在**无损格式图像 (如 PNG, BMP)** 中安全地隐藏或恢复文本文件。

> **⚠️ 重要兼容性警告**
>
> **v4.0.0** 引入了对数据存储格式的重大修复，以支持隐藏大于 64KB 的文件。因此，此版本**无法恢复**由 v3.0.0 或更早版本创建的隐藏数据。
>
> 请使用 v4.0.0 重新隐藏您的重要数据。

## ✨ 功能

- **隐藏文本**: 将任意文本文件的内容嵌入到无损格式图像的像素数据中。
- **恢复文本**: 从已嵌入信息的图像中提取并恢复原始文本文件。
- **智能默认值**: 自动为输出文件生成合理的文件名，简化常用操作。
- **安全覆盖**: 默认防止意外覆盖现有文件，并提供 `--force` 选项进行强制写入。
- **多格式支持**: 支持多种常用无损图像格式，包括 **PNG, BMP, TIFF, QOI 和 WebP**。
- **跨平台**: 支持在 Windows 和 Linux 上编译和运行。
- **简单易用**: 提供清晰的命令行接口和错误提示。

> **重要提示**: LSB 隐写术要求图像格式是**无损的**。请确保在使用 TIFF 或 WebP 格式时，它们被保存为无损模式，否则隐藏的信息将无法恢复。

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

使用 `hide` 命令将文本文件隐藏到图像中。

```bash
lsb_hide hide --image <原始图像.png> --text <要隐藏的文本.txt> [OPTIONS]
```

**参数说明:**

- `-i, --image <IMAGE>`: 原始的无损格式图像文件路径 (例如 .png)。
- `-t, --text <TEXT>`: 要隐藏的文本文件路径。
- `-d, --dest <DEST>`: **[可选]** 嵌入信息后要保存的新图像文件路径。如果未提供，将默认保存到 `doctored_{原始文件名}`。
- `--force`: **[可选]** 如果目标文件已存在，强制覆盖它。

**示例:**

```bash
# 简单用法，自动生成输出文件名 "doctored_input.png"
lsb_hide hide -i input.png -t secret.txt

# 指定输出文件名
lsb_hide hide -i input.png -t secret.txt -d output.png

# 如果 output.png 已存在，强制覆盖它
lsb_hide hide -i input.png -t secret.txt -d output.png --force
```

### 恢复文本

使用 `recover` 命令从图像中恢复隐藏的文本。

```bash
lsb_hide recover --image <已嵌入信息的图像.png> [OPTIONS]
```

**参数说明:**

- `-i, --image <IMAGE>`: 包含隐藏信息的图像文件路径。
- `-t, --text <TEXT>`: **[可选]** 恢复出的文本要保存到的文件路径。如果未提供，将默认保存到 `recovered_{原始文件名}.txt`。
- `--force`: **[可选]** 如果目标文件已存在，强制覆盖它。

**示例:**

```bash
# 简单用法，自动生成输出文件名 "recovered_output.txt"
lsb_hide recover -i output.png

# 指定输出文件名
lsb_hide recover -i output.png -t recovered.txt

# 如果 recovered.txt 已存在，强制覆盖它
lsb_hide recover -i output.png -t recovered.txt --force
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
