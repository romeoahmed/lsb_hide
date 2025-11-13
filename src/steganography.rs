use std::io::{self, ErrorKind};

/// 隐藏一个 64 位值 (`value`) 到像素数组 (`pix`) 的指定区域。
///
/// 隐写采用 LSB (最低有效位) 机制，使用像素字节的最低两位 (`& 0x3`) 来存储数据。
/// 每个像素字节可以存储 2 bits 的数据，因此 `size` 字节可存储 `size * 2` bits。
/// 数据是按小端序 (Little-Endian) 方式写入的：`value` 的最低位写入 `sub_pix` 的第一个字节。
///
/// # Arguments
///
/// * `value` - 要隐藏的 64 位无符号整数。
/// * `pix` - 包含图像像素数据的可变字节切片。
/// * `dix` - 数据开始隐写的索引偏移量 (Data Index)，应跳过 BMP 头。
/// * `size` - 用于隐写的字节数 (像素字节数)。
///
/// # Errors
///
/// 如果隐写区域 `dix` 到 `dix + size` 超出了 `pix` 的边界，将返回 `ErrorKind::InvalidInput` 错误。
pub fn modify(mut value: u64, pix: &mut [u8], dix: usize, size: usize) -> Result<(), io::Error> {
    if dix.checked_add(size).map_or(true, |end| end > pix.len()) {
        return Err(io::Error::new(
            ErrorKind::InvalidInput,
            "Steganography region out of bounds.",
        ));
    }

    let sub_pix = &mut pix[dix..(dix + size)];

    for byte in sub_pix.iter_mut() {
        *byte = ((value & 0x3) as u8) | (*byte & 0xFC);
        value >>= 2;
    }

    Ok(())
}

/// 从像素数组 (`pix`) 的指定区域恢复一个 64 位值。
///
/// 从每个像素字节的最低两位 (`& 0x3`) 中提取数据，并按照小端序 (Little-Endian)
/// 方式组合成一个 64 位整数。
///
/// # Arguments
///
/// * `pix` - 包含图像像素数据的字节切片。
/// * `dix` - 数据开始恢复的索引偏移量 (Data Index)，应跳过 BMP 头。
/// * `size` - 用于恢复的字节数 (像素字节数)。
///
/// # Returns
///
/// 成功时返回恢复的 `u64` 值。
///
/// # Errors
///
/// * 如果恢复区域 `dix` 到 `dix + size` 超出了 `pix` 的边界，将返回 `ErrorKind::InvalidInput` 错误。
/// * 如果 `size` 大于 32 字节，由于 u64 只有 64 bits (32 bytes * 2 bits/byte)，将返回 `ErrorKind::InvalidInput` 错误。
pub fn recover(pix: &[u8], dix: usize, size: usize) -> Result<u64, io::Error> {
    if dix.checked_add(size).map_or(true, |end| end > pix.len()) {
        return Err(io::Error::new(
            ErrorKind::InvalidInput,
            "Extraction area out of bounds.",
        ));
    }

    let mut result: u64 = 0;
    if size > 32 {
        return Err(io::Error::new(
            ErrorKind::InvalidInput,
            "Extraction size limit exceeded (max 32 bytes for a u64 value).",
        ));
    }

    let sub_pix = &pix[dix..(dix + size)];

    for (i, &byte) in sub_pix.iter().enumerate() {
        result |= ((byte & 0x3) as u64) << (i * 2);
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::*;

    /// 一个完整的端到端测试，模拟隐藏和恢复过程。
    #[test]
    fn test_hide_and_recover_e2e() {
        // 1. 准备测试数据
        // 模拟一个 BMP 文件的头部和足够大的数据区
        let mut picture = vec![0u8; 1024];
        // 填充一些伪随机数据，模拟真实的图像像素
        for (i, byte) in picture.iter_mut().enumerate() {
            *byte = (i % 256) as u8;
        }

        // 模拟要隐藏的文本
        let original_text = "Hello, Steganography! 你好，隐写术！";
        let text_bytes = original_text.as_bytes();
        let text_len = text_bytes.len() as u64;

        // 2. 隐藏数据
        // 隐藏文本长度
        modify(
            text_len,
            &mut picture,
            BMP_HEADER_SIZE,
            LENGTH_HIDING_BYTES,
        )
        .expect("Failed to hide text length.");

        // 逐字节隐藏文本内容
        for (i, &char_byte) in text_bytes.iter().enumerate() {
            let offset = BMP_HEADER_SIZE + LENGTH_HIDING_BYTES + BYTES_PER_CHAR * i;
            modify(char_byte as u64, &mut picture, offset, BYTES_PER_CHAR)
                .expect("Failed to hide a character.");
        }

        // 3. 恢复数据
        // 恢复文本长度
        let recovered_len = recover(&picture, BMP_HEADER_SIZE, LENGTH_HIDING_BYTES)
            .expect("Failed to recover text length.");

        // 断言长度一致
        assert_eq!(text_len, recovered_len, "Recovered length should match original length.");

        // 逐字节恢复文本内容
        let recovered_bytes: Vec<u8> = (0..recovered_len as usize)
            .map(|i| {
                let offset = BMP_HEADER_SIZE + LENGTH_HIDING_BYTES + BYTES_PER_CHAR * i;
                recover(&picture, offset, BYTES_PER_CHAR)
                    .map(|val| val as u8)
                    .expect("Failed to recover a character.")
            })
            .collect();

        // 4. 断言结果
        // 断言恢复的文本内容与原始文本完全一致
        assert_eq!(text_bytes, recovered_bytes.as_slice(), "Recovered text should match original text.");
    }

    /// 测试 recover 函数在数据不足时能否正确返回错误
    #[test]
    fn test_recover_not_enough_data() {
        // 只有 7 个字节，但我们需要 8 个字节来恢复一个 u64
        let picture = vec![0u8; 7];
        let result = recover(&picture, 0, 8);

        // 断言结果是 Err
        assert!(result.is_err(), "Recover should fail when there is not enough data.");
    }
}
