use image::{ImageBuffer, Rgba};
use lsb_hide::{
    cli::{HideArgs, RecoverArgs},
    handler::{handle_hide, handle_recover},
};
use rand::RngCore;
use std::fs;
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
