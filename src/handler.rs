use crate::cli::{HideArgs, RecoverArgs};
use crate::constants::{BMP_HEADER_SIZE, BYTES_PER_CHAR, LENGTH_HIDING_BYTES};
use crate::steganography::{modify, recover};
use anyhow::{Context, Result};
use colored::Colorize;
use std::fs;

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

pub fn handle_recover(args: RecoverArgs) -> Result<()> {
    let picture = fs::read(&args.image).with_context(|| {
        format!(
            "Unable to read image file: {}",
            args.image.to_string_lossy().red().bold()
        )
    })?;

    let text_len = recover(&picture, BMP_HEADER_SIZE, LENGTH_HIDING_BYTES).with_context(|| {
        format!(
            "An error occurred while recovering the text length: {}",
            args.text.to_string_lossy().red().bold()
        )
    })?;

    let mut text: Vec<u8> = vec![0; text_len as usize];

    for i in 0..(text_len as usize) {
        let offset = BMP_HEADER_SIZE + LENGTH_HIDING_BYTES + BYTES_PER_CHAR * i;

        let value = recover(&picture, offset, BYTES_PER_CHAR)
                    .with_context(|| format!("An error occurred while recovering the current index character and offset: ({}, {})", i.to_string().red().bold(), offset.to_string().red().bold()))?;

        text[i] = value as u8;
    }

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
