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
