//! # 命令处理逻辑模块
//!
//! 包含处理 `hide` 和 `recover` 子命令的高级业务逻辑。
//! 本模块负责协调文件 I/O、调用核心隐写算法以及向用户报告结果。

use crate::cli::{HideArgs, RecoverArgs};
use crate::constants::{BYTES_PER_CHAR, LENGTH_HIDING_BYTES};
use crate::steganography::{modify, recover};
use anyhow::{Context, Result};
use colored::Colorize;
use image::{GenericImageView, ImageBuffer, Rgba};
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
pub fn handle_hide(args: HideArgs) -> Result<()> {
    // 读取源图像和待隐藏的文本文件
    // let mut picture = fs::read(&args.image).with_context(|| {
    //     format!(
    //         "Unable to read image file: {}",
    //         args.image.to_string_lossy().red().bold()
    //     )
    // })?;
    let img = image::open(&args.image).with_context(|| {
        format!(
            "Unable to read image file: {}",
            args.image.to_string_lossy().red().bold()
        )
    })?;

    let (width, height) = img.dimensions();
    let mut picture_bytes = img.into_rgba8().into_raw();

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
    modify(text_len, &mut picture_bytes, 0, LENGTH_HIDING_BYTES).with_context(|| {
        "Failed to hide the message length in the image. \nThe image file may be corrupt or write-protected."
    })?;

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

    // fs::write(&args.dest, picture).with_context(|| {
    //     format!(
    //         "Unable to write to target image file: {}",
    //         args.dest.to_string_lossy().red().bold()
    //     )
    // })?;

    let output_img = ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(width, height, picture_bytes)
        .with_context(|| "Failed to create image buffer from modified bytes.")?;

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
pub fn handle_recover(args: RecoverArgs) -> Result<()> {
    // let picture = fs::read(&args.image).with_context(|| {
    //     format!(
    //         "Unable to read image file: {}",
    //         args.image.to_string_lossy().red().bold()
    //     )
    // })?;
    let img = image::open(&args.image).with_context(|| {
        format!(
            "Unable to read image file: {}",
            args.image.to_string_lossy().red().bold()
        )
    })?;

    let picture_bytes = img.into_rgba8().into_raw();

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
        .collect::<Result<Vec<u8>>>()?;

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
