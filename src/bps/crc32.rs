//  BPS file wrapper
use crate::errors::Errors;
use crc32fast::Hasher;
use std::io::{
    prelude::*,
    SeekFrom,
};
use std::fmt;
use std::fs::File;
use super::FOOTER_SIZE;

const SIZE_OF_U32: usize = std::mem::size_of::<u32>();

#[derive(Debug)]
pub struct Crc32 {
    crc32: u32,
}

impl Crc32 {
    fn new(crc32: u32) -> Self {
        Self {
            crc32,
        }
   }

    // Compare the Crc32 against the File
    pub fn compare(&self, file: &mut File) -> Result<(), Errors> {
        let mut buf: [u8; 32 * 1_024] = [0; 32 * 1_024];
        let mut hasher = Hasher::new();

        loop {
            match file.read(&mut buf)? {
                0 => break,
                n => hasher.update(&buf[..n]),
            }
        }

        let hash = hasher.finalize();

        if hash != self.crc32 {
            return Err(Errors::BadCrc32);
        }

        Ok(())
    }
}

pub struct BpsCrc32 {
    bps: u32,
    source: u32,
    target: u32,
}

impl BpsCrc32 {
    pub fn bps(&self) -> u32 {
        self.bps
    }

    pub fn source(&self) -> u32 {
        self.source
    }

    pub fn target(&self) -> u32 {
        self.target
    }
}

impl fmt::Debug for BpsCrc32 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("BpsCrc32")
            .field("bps", &hex::encode(self.bps.to_le_bytes()))
            .field("source", &hex::encode(self.source.to_le_bytes()))
            .field("target", &hex::encode(self.target.to_le_bytes()))
            .finish()
    }
}

impl fmt::Display for BpsCrc32 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "BPS: {}, ROM: {}, TARGET: {}",
            hex::encode(self.bps.to_le_bytes()),
            hex::encode(self.source.to_le_bytes()),
            hex::encode(self.target.to_le_bytes()),
        )
    }
}

impl TryFrom<&mut File> for BpsCrc32 {
    type Error = Errors;

    fn try_from(file: &mut File) -> Result<Self, Self::Error> {
        let file_size = file.metadata()?.len();

        if file_size < FOOTER_SIZE as u64 {
            return Err(Errors::BadBps);
        }

        // Reach each set of bytes into the buffer individually, before casting
        // to u32.
        // We could read the entire footer at the same time, but this avoids
        // having to use a try_from to get the type that from_le_bytes expects.
        let footer_position = file_size - FOOTER_SIZE as u64;
        let mut buf: [u8; SIZE_OF_U32] = [0; SIZE_OF_U32];
        file.seek(SeekFrom::Start(footer_position))?;

        file.read_exact(&mut buf)?;
        let source = u32::from_le_bytes(buf);

        file.read_exact(&mut buf)?;
        let target = u32::from_le_bytes(buf);

        file.read_exact(&mut buf)?;
        let bps = u32::from_le_bytes(buf);

        let crc32 = Self {
            bps,
            source,
            target,
        };

        println!("{crc32}");

        Ok(crc32)
    }
}
