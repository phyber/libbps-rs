//  BPS file wrapper
use crate::errors::Errors;
use std::io::prelude::*;
use std::fs::File;

// This decodes a number from the BPS file
pub fn varint(f: &mut File) -> Result<u64, Errors> {
    let mut data: u64 = 0;
    let mut shift: u64 = 1;
    let mut buf: [u8; 1] = [0; 1];

    loop {
        f.read_exact(&mut buf)?;
        let x = u64::from(buf[0]);

        data += (x & 0x7f) * shift;

        if x & 0x80 != 0 {
            break;
        }

        shift <<= 7;
        data += shift;
    }

    Ok(data)
}
