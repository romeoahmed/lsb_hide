/// BMP 文件的标准头部大小 (字节)。
/// 隐写操作将跳过这个头部，从像素数据开始。
pub const BMP_HEADER_SIZE: usize = 54;

/// 用于隐写文本长度信息的字节数。
/// 由于 `u64` 占用 8 字节 (64 bits)，而每个像素字节存储 2 bits，
/// 因此需要 64 / 2 = 32 个像素字节来隐藏文本长度。
pub const LENGTH_HIDING_BYTES: usize = 32;

/// 用于隐写文本中单个字符的字节数。
/// 每个字符按 `u8` (8 bits) 处理，需要 8 / 2 = 4 个像素字节。
pub const BYTES_PER_CHAR: usize = 4;
