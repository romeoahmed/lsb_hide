use std::io::{self, ErrorKind};

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
