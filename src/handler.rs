//! # 命令处理逻辑模块
//!
//! 包含处理 `hide` 和 `recover` 子命令的高级业务逻辑。
//! 本模块负责协调文件 I/O、调用核心隐写算法以及向用户报告结果。

use crate::cli::{HideArgs, RecoverArgs};
use crate::constants::{BYTES_PER_CHAR, LENGTH_HIDING_BYTES};
use crate::steganography::{modify, recover};
use anyhow::Context;
use colored::Colorize;
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgb, Rgba};
use std::fs;

/// 处理 'Hide' 命令的执行逻辑。
///
/// 负责读取图像和文本文件、检查隐写空间是否足够、调用隐写核心函数隐藏长度和字符，
/// 最后将结果写入目标图像文件。
///
/// # Arguments
///
/// * `args` - 包含输入/输出路径的 `HideArgs` 结构体。
///
/// # Errors
///
/// 如果发生以下任一情况，将返回错误：
/// * 无法读取输入的图像或文本文件。
/// * 图像文件没有足够的空间来隐藏文本。
/// * 核心隐写函数 (`modify`) 在执行过程中失败。
/// * 无法写入到目标图像文件。
pub fn handle_hide(args: HideArgs) -> anyhow::Result<()> {
    // 读取源图像
    let img = image::open(&args.image).with_context(|| {
        format!(
            "Unable to read image file: {}",
            args.image.to_string_lossy().red().bold()
        )
    })?;

    let (width, height) = img.dimensions();

    // 将图像转换为字节流，判断并记录原始颜色格式（RGB/RGBA）
    let (mut picture_bytes, is_rgba) = match img {
        DynamicImage::ImageRgba8(rgba) => (rgba.into_raw(), true),
        _ => (img.into_rgb8().into_raw(), false),
    };

    let text = fs::read(&args.text).with_context(|| {
        format!(
            "Unable to read text file: {}",
            args.text.to_string_lossy().red().bold()
        )
    })?;

    // 检查图像是否有足够的空间来隐藏文本
    let required_space = text.len() * BYTES_PER_CHAR;
    let available_space = picture_bytes.len().saturating_sub(LENGTH_HIDING_BYTES);

    anyhow::ensure!(
        available_space >= required_space,
        "Not enough space in the image to hide the text. \nRequired: {}, Available: {}",
        required_space.to_string().red().bold(),
        available_space.to_string().green().bold()
    );

    // 隐藏文本长度
    let text_len = text.len() as u64;
    modify(text_len, &mut picture_bytes, 0, LENGTH_HIDING_BYTES).context(
        "Failed to hide the message length in the image. \nThe image file may be corrupt or write-protected."
    )?;

    // 逐字节隐藏文本内容
    text.iter().enumerate().try_for_each(|(i, &char_byte)| {
        let offset = LENGTH_HIDING_BYTES + BYTES_PER_CHAR * i;
        modify(char_byte as u64, &mut picture_bytes, offset, BYTES_PER_CHAR).with_context(|| {
            let char_info = std::str::from_utf8(&[char_byte])
                .map(ToString::to_string)
                .unwrap_or_else(|_| {
                    format!("byte value {}", char_byte)
                });
            format!(
                "Failed to hide character {} (at index {}). \nThe image might not have enough capacity or is corrupted.",
                char_info.red().bold(),
                i.to_string().green()
            )
        })
    })?;

    // 根据原始颜色格式（RGB/RGBA），从修改后的字节创建 DynamicImage
    let output_img = if is_rgba {
        ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(width, height, picture_bytes)
            .context("Failed to create RGBA image buffer from modified bytes.")
            .map(DynamicImage::ImageRgba8)
    } else {
        ImageBuffer::<Rgb<u8>, Vec<u8>>::from_raw(width, height, picture_bytes)
            .context("Failed to create RGB image buffer from modified bytes.")
            .map(DynamicImage::ImageRgb8)
    }?;

    output_img.save(&args.dest).with_context(|| {
        format!(
            "Unable to write to target image file: {}",
            args.dest.to_string_lossy().red().bold()
        )
    })?;

    println!(
        "The text has been successfully hidden and saved: {}",
        args.dest.to_string_lossy().green().bold()
    );

    Ok(())
}

/// 处理 'Recover' 命令的执行逻辑。
///
/// 负责读取经过隐写的图像文件、调用恢复核心函数获取文本长度和每个字符，
/// 最后将恢复的文本内容写入目标文本文件。
///
/// # Arguments
///
/// * `args` - 包含输入/输出路径的 `RecoverArgs` 结构体。
///
/// # Errors
///
/// 如果发生以下任一情况，将返回错误：
/// * 无法读取输入的图像文件。
/// * 核心恢复函数 (`recover`) 在执行过程中失败。
/// * 无法写入到目标文本文件。
pub fn handle_recover(args: RecoverArgs) -> anyhow::Result<()> {
    // 读取图像文件
    let img = image::open(&args.image).with_context(|| {
        format!(
            "Unable to read image file: {}",
            args.image.to_string_lossy().red().bold()
        )
    })?;

    // 根据原始颜色格式（RGB/RGBA），将图像转换为字节流
    let picture_bytes = match img {
        DynamicImage::ImageRgba8(rgba) => rgba.into_raw(),
        _ => img.into_rgb8().into_raw(),
    };

    // 恢复隐藏文本的长度
    let text_len = recover(&picture_bytes, 0, LENGTH_HIDING_BYTES).with_context(|| {
        format!(
            "Failed to recover message length from '{}'. \nThe image may not contain a hidden message or is corrupted.",
            args.image.to_string_lossy().red().bold()
        )
    })?;

    // 根据恢复的长度，逐字节恢复文本内容
    let text: Vec<u8> = (0..text_len as usize)
        .map(|i| {
            let offset = LENGTH_HIDING_BYTES + BYTES_PER_CHAR * i;
            recover(&picture_bytes, offset, BYTES_PER_CHAR)
                .map(|value| value as u8)
                .with_context(|| {
                    format!(
                        "Failed to recover character at index {}. \nThe data at offset {} appears to be corrupted or invalid.",
                        i.to_string().red().bold(),
                        offset.to_string().red().bold()
                    )
                })
        })
        .collect::<anyhow::Result<Vec<u8>>>()?;

    fs::write(&args.text, text).with_context(|| {
        format!(
            "Unable to write to target text file: {}",
            args.text.to_string_lossy().red().bold()
        )
    })?;

    println!(
        "The text has been successfully recovered and saved: {}",
        args.text.to_string_lossy().green().bold()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::{HideArgs, RecoverArgs};
    use rand::RngCore;
    use std::path::PathBuf;
    use tempfile::tempdir;

    /// 一个辅助函数，用于创建一个带有随机像素的测试图像
    fn create_test_image(path: &PathBuf, width: u32, height: u32) {
        let mut img_buf = ImageBuffer::new(width, height);
        let mut raw_pixels = vec![0u8; (width * height * 4) as usize];
        rand::rng().fill_bytes(&mut raw_pixels);

        img_buf
            .pixels_mut()
            .zip(raw_pixels.chunks_exact(4))
            .for_each(|(pixel, chunk)| {
                *pixel = Rgba([chunk[0], chunk[1], chunk[2], 255]);
            });

        img_buf.save(path).expect("Failed to create test image.");
    }

    /// 验证从隐藏到恢复的完整流程
    #[test]
    fn test_handle_hide_and_recover_integration() -> anyhow::Result<()> {
        // 1. 准备环境
        let dir = tempdir()?;
        let original_image_path = dir.path().join("original.png");
        let hidden_image_path = dir.path().join("hidden.png");
        let source_text_path = dir.path().join("source.txt");
        let recovered_text_path = dir.path().join("recovered.txt");

        create_test_image(&original_image_path, 100, 100);
        let original_text = "This is a test message for the handler! 这是一个给处理器的测试信息！";
        fs::write(&source_text_path, original_text)?;

        // 2. 测试 handle_hide
        let hide_args = HideArgs {
            image: original_image_path.clone(),
            text: source_text_path.clone(),
            dest: hidden_image_path.clone(),
        };
        handle_hide(hide_args)?;
        assert!(
            hidden_image_path.exists(),
            "Hidden image should be created."
        );

        // 3. 测试 handle_recover
        let recover_args = RecoverArgs {
            image: hidden_image_path.clone(),
            text: recovered_text_path.clone(),
        };
        handle_recover(recover_args)?;
        assert!(
            recovered_text_path.exists(),
            "Recovered text file should be created."
        );

        // 4. 验证结果
        let recovered_text = fs::read_to_string(&recovered_text_path)?;
        assert_eq!(
            original_text, recovered_text,
            "Recovered text must match the original."
        );

        Ok(())
    }

    /// 验证空间不足时的错误处理
    #[test]
    fn test_handle_hide_not_enough_space() {
        // 1. 准备环境
        let dir = tempdir().unwrap();
        let image_path = dir.path().join("small.png");
        let text_path = dir.path().join("large.txt");
        let dest_path = dir.path().join("dest.png");

        // 创建一个非常小的图片
        create_test_image(&image_path, 10, 10);
        // 创建一个非常大的文本
        let large_text = "a".repeat(5000);
        fs::write(&text_path, large_text).unwrap();

        // 2. 执行并断言错误
        let hide_args = HideArgs {
            image: image_path,
            text: text_path,
            dest: dest_path,
        };
        let result = handle_hide(hide_args);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Not enough space"));
    }
}
