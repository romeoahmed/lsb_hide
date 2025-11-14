use crate::cli::{HideArgs, RecoverArgs};
use crate::constants::{BMP_HEADER_SIZE, BYTES_PER_CHAR, LENGTH_HIDING_BYTES};
use crate::steganography::{modify, recover};
use anyhow::{Context, Result};
use colored::Colorize;
use std::fs;

/// 处理 'Hide' 命令的执行逻辑。
///
/// 负责读取图像和文本文件、检查隐写空间是否足够、调用隐写核心函数隐藏长度和字符，
/// 最后将结果写入目标图像文件。
///
/// # Arguments
///
/// * `args` - 包含输入/输出路径的 `HideArgs` 结构体。
pub fn handle_hide(args: HideArgs) -> Result<()> {
    let mut picture = fs::read(&args.image).with_context(|| {
        format!(
            "Unable to read image file: {}",
            args.image.to_string_lossy().red().bold()
        )
    })?;

    let text = fs::read(&args.text).with_context(|| {
        format!(
            "Unable to read text file: {}",
            args.text.to_string_lossy().red().bold()
        )
    })?;

    let required_space = text.len() * BYTES_PER_CHAR;
    let available_space = picture
        .len()
        .saturating_sub(BMP_HEADER_SIZE + LENGTH_HIDING_BYTES);

    if available_space < required_space {
        anyhow::bail!(
            "The text file ({}) is too long, and there is insufficient image space (only {}).",
            required_space.to_string().red().bold(),
            available_space.to_string().red().bold()
        );
    }

    let text_len = text.len() as u64;

    modify(text_len, &mut picture, BMP_HEADER_SIZE, LENGTH_HIDING_BYTES).with_context(|| {
        format!(
            "An error occurred while hiding the text length: {}",
            text_len.to_string().red().bold()
        )
    })?;

    for (i, &char_byte) in text.iter().enumerate() {
        let offset = BMP_HEADER_SIZE + LENGTH_HIDING_BYTES + BYTES_PER_CHAR * i;

        let char_u64 = char_byte as u64;

        modify(char_u64, &mut picture, offset, BYTES_PER_CHAR).with_context(|| {
            format!(
                "An error occurred while hiding the current index character: {}",
                i.to_string().red().bold()
            )
        })?;
    }

    fs::write(&args.dest, picture).with_context(|| {
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
pub fn handle_recover(args: RecoverArgs) -> Result<()> {
    let picture = fs::read(&args.image).with_context(|| {
        format!(
            "Unable to read image file: {}",
            args.image.to_string_lossy().red().bold()
        )
    })?;

    let text_len = recover(&picture, BMP_HEADER_SIZE, LENGTH_HIDING_BYTES).with_context(|| {
        format!(
            "An error occurred while recovering the text length from image: {}",
            args.image.to_string_lossy().red().bold()
        )
    })?;

    let text: Vec<u8> = (0..text_len as usize)
        .map(|i| {
            let offset = BMP_HEADER_SIZE + LENGTH_HIDING_BYTES + BYTES_PER_CHAR * i;
            recover(&picture, offset, BYTES_PER_CHAR)
                .map(|value| value as u8)
                .with_context(|| {
                    format!(
                        "An error occurred while recovering character at index {} (offset {})",
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
