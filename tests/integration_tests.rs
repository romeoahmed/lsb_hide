use anyhow::Ok;
use image::{ImageBuffer, Rgba};
use lsb_hide::{
    cli::{HideArgs, RecoverArgs},
    handler::{handle_hide, handle_recover},
};
use rand::RngCore;
use std::fs;
use std::path::Path;
use tempfile::tempdir;

/// 一个辅助函数，用于创建一个带有随机像素的测试图像
fn create_test_image(path: &Path, width: u32, height: u32) {
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
        dest: Some(hidden_image_path.clone()),
        force: false,
    };
    handle_hide(hide_args)?;
    assert!(
        hidden_image_path.exists(),
        "Hidden image should be created."
    );

    // 3. 测试 handle_recover
    let recover_args = RecoverArgs {
        image: hidden_image_path.clone(),
        text: Some(recovered_text_path.clone()),
        force: false
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

/// 验证当用户不提供输出路径时，是否能正确生成默认路径并完成操作
#[test]
fn test_handle_hide_and_recover_with_defaults() -> anyhow::Result<()> {
    // 1. 准备环境
    let dir = tempdir()?;
    let original_image_path = dir.path().join("original.png");
    let source_text_path = dir.path().join("source.txt");

    create_test_image(&original_image_path, 100, 100);
    let original_text = "Testing default path generation. 测试默认路径生成。";
    fs::write(&source_text_path, original_text)?;

    // 2. 测试 handle_hide，不提供 dest 路径
    let hide_args = HideArgs {
        image: original_image_path.clone(),
        text: source_text_path.clone(),
        dest: None, // 关键：测试 None 的情况
        force: false
    };
    handle_hide(hide_args)?;

    // 验证默认的隐藏图像文件是否已创建
    let expected_hidden_path = dir.path().join("doctored_original.png");
    assert!(
        expected_hidden_path.exists(),
        "Default hidden image should be created at: {:?}",
        expected_hidden_path
    );

    // 3. 测试 handle_recover，不提供 text 输出路径
    let recover_args = RecoverArgs {
        image: expected_hidden_path, // 使用上一步生成的默认文件
        text: None,                  // 关键：测试 None 的情况
        force: false
    };
    handle_recover(recover_args)?;

    // 验证默认的恢复文本文件是否已创建
    let expected_recovered_path = dir.path().join("recovered_doctored_original.txt");
    assert!(
        expected_recovered_path.exists(),
        "Default recovered text file should be created at: {:?}",
        expected_recovered_path
    );

    // 4. 验证结果
    let recovered_text = fs::read_to_string(&expected_recovered_path)?;
    assert_eq!(
        original_text, recovered_text,
        "Recovered text from default file must match the original."
    );

    Ok(())
}

/// 验证覆盖保护机制以及 `--force` 标志是否按预期工作
#[test]
fn test_overwrite_protection_and_force_flag() -> anyhow::Result<()> {
    // 1. 准备环境
    let dir = tempdir()?;
    let image_path = dir.path().join("image.png");
    let text_path = dir.path().join("text.txt");
    let dest_path = dir.path().join("dest.png");

    create_test_image(&image_path, 50, 50);
    fs::write(&text_path, "some text")?;

    // 2. 场景一：测试覆盖保护
    // 先创建一个同名的目标文件，模拟“文件已存在”的场景
    fs::write(&dest_path, "this is a dummy file to be overwritten")?;
    assert!(dest_path.exists());

    // 构建参数，不使用 --force
    let hide_args_no_force = HideArgs {
        image: image_path.clone(),
        text: text_path.clone(),
        dest: Some(dest_path.clone()),
        force: false,
    };

    // 执行并断言操作会失败
    let result = handle_hide(hide_args_no_force);
    assert!(result.is_err(), "Execution should fail without --force when file exists.");
    if let Err(e) = result {
        assert!(e.to_string().contains("Output file already exists"));
    }

    // 3. 场景二：测试强制覆盖
    // 构建参数，这次使用 --force
    let hide_args_with_force = HideArgs {
        image: image_path.clone(),
        text: text_path.clone(),
        dest: Some(dest_path.clone()),
        force: true,
    };

    // 执行并断言操作会成功
    let result = handle_hide(hide_args_with_force);
    assert!(result.is_ok(), "Execution should succeed with --force when file exists.");

    // 验证文件确实被覆盖（内容不再是 "this is a dummy file..."）
    let dummy_content = fs::read(&dest_path)?;
    assert_ne!(dummy_content, b"this is a dummy file to be overwritten");

    Ok(())
}

/// 验证空间不足时的错误处理
#[test]
fn test_handle_hide_not_enough_space() -> anyhow::Result<()> {
    // 1. 准备环境
    let dir = tempdir()?;
    let image_path = dir.path().join("small.png");
    let text_path = dir.path().join("large.txt");
    let dest_path = dir.path().join("dest.png");

    // 创建一个非常小的图片
    create_test_image(&image_path, 10, 10);
    // 创建一个非常大的文本
    let large_text = "a".repeat(5000);
    fs::write(&text_path, large_text)?;

    // 2. 执行并断言错误
    let hide_args = HideArgs {
        image: image_path,
        text: text_path,
        dest: Some(dest_path),
        force: false
    };
    let result = handle_hide(hide_args);

    assert!(result.is_err());
    if let Err(e) = result {
        assert!(e.to_string().contains("Not enough space"));
    }

    Ok(())
}
