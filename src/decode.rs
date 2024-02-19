//  BPS file wrapper
use crate::errors::Errors;
use std::io::{
    prelude::*,
    SeekFrom,
};
use std::fs::File;

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

    println!("{data:?}");
    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_BPS_PATH: &str = "test-data/Grand_Poo_World_3.bps";
}
