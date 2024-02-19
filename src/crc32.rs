//  BPS file wrapper
use crate::action::Action;
use crate::errors::Errors;
use crc32fast::Hasher;
use std::io::{
    prelude::*,
    SeekFrom,
};
use std::fmt;
use std::fs::File;
use std::path::Path;

mod crc32;
use crc32::BpsCrc32;

const MAGIC: &[u8; 4] = b"BPS1";
const MAGIC_SIZE: usize = MAGIC.len();
const SIZE_OF_U32: usize = std::mem::size_of::<u32>();
const SIZE_OF_U64: usize = std::mem::size_of::<u64>();

// Header size is dynamic. It consists of:
//  - Magic bytes
//  - Source ROM size
//  - Target (output) size
//  - Metadata size
//  - Metadata, of the size from the previous components
//    - This size is dynamic.
//
// The final size of the header will be the 3 statically sized components +
// the dynamic sized metadata.
const HEADER_BASE_SIZE: usize = MAGIC_SIZE + (SIZE_OF_U64 * 3);

// Footer consists of 3 CRC32 checksums in the order: source, target, bps.
// Each checksum is 4 bytes each (32bit).
const FOOTER_SIZE: usize = 12;

fn decode_varint(f: &mut File) -> Result<u64, Errors> {
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

#[derive(Debug)]
struct BpsHeader {
    source_size: u64,
    target_size: u64,
    metadata_size: u64,
    header_size: u64,
}

impl BpsHeader {
    // Checks that the header is correct
    fn magic_check(f: &mut File) -> Result<(), Errors> {
        let mut buf: [u8; MAGIC_SIZE] = [0; MAGIC_SIZE];

        f.seek(SeekFrom::Start(0))?;
        f.read_exact(&mut buf)?;

        if &buf != MAGIC {
            return Err(Errors::BadHeader);
        }

        Ok(())
    }

    fn source_size(&self) -> u64 {
        self.source_size
    }

    fn target_size(&self) -> u64 {
        self.target_size
    }
}

impl TryFrom<&mut File> for BpsHeader {
    type Error = Errors;

    fn try_from(file: &mut File) -> Result<Self, Self::Error> {
        Self::magic_check(file)?;

        let mut header_size: u64 = HEADER_BASE_SIZE as u64;

        let source_size = decode_varint(file)?;
        let target_size = decode_varint(file)?;
        let metadata_size = decode_varint(file)?;

        // Skip over metadata
        if metadata_size > 0 {
            file.seek(SeekFrom::Current(metadata_size as i64))?;
            header_size += metadata_size;
        }

        let header = BpsHeader {
            source_size,
            target_size,
            metadata_size,
            header_size,
        };

        Ok(header)
    }
}

#[derive(Debug)]
pub struct Bps {
    file: File,
    header: BpsHeader,
    crc32: BpsCrc32,
    patch_size: u64,
}

impl Bps {
    pub fn new<P>(path: &P) -> Result<Self, Errors>
    where
        P: AsRef<Path> + ?Sized,
    {
        let mut file = File::open(path)?;
        let patch_size = file.metadata()?.len();
        let crc32 = BpsCrc32::try_from(&mut file)?;

        // We can now check the CRC32 of the BPS file.
        let mut hasher = Hasher::new();
        let mut buf: [u8; 32 * 1_024] = [0; 32 * 1_024];
        let mut total_read = 0;
        file.seek(SeekFrom::Start(0))?;

        loop {
            match file.read(&mut buf)? {
                0 => break,
                mut n => {
                    // The CRC for the BPS file is the sum of all bytes, except
                    // for the last 4.
                    // This is probably a really bad way of achieving this and
                    // I expect that there are times this would break.
                    total_read += n as u64;

                    if total_read == patch_size {
                        n -= 4;
                    }

                    hasher.update(&buf[..n]);
                },
            }
        }

        let hash = hasher.finalize();

        if hash != crc32.bps() {
            println!(
                "{} != {}",
                hex::encode(hash.to_le_bytes()),
                hex::encode(crc32.bps().to_le_bytes()),
            );
            println!("{hash} != {}", crc32.bps());
            return Err(Errors::BadCrc32Bps);
        }

        // Now that we know the CRC32 is good, read the header.
        let header = BpsHeader::try_from(&mut file)?;

        // Set the BPS position to after the header.
        file.seek(SeekFrom::Start(header.header_size))?;

        let mut bps = Self {
            file,
            header,
            crc32,
            patch_size,
        };

        Ok(bps)
    }

    fn current_position(&mut self) -> Result<u64, Errors> {
        let pos = self.file.seek(SeekFrom::Current(0))?;
        Ok(pos)
    }

    // Returns the next action, advancing the BPS file position.
    pub fn action(&mut self) -> Result<Action, Errors> {
        let instruction = decode_varint(&mut self.file)?;
        let action = Action::from(instruction);

        Ok(action)
    }

    pub fn patch_size(&self) -> u64 {
        self.patch_size
    }

    pub fn read_len_at(
        &mut self,
        n: usize,
        seek: SeekFrom,
    ) -> Result<Vec<u8>, Errors> {
        let mut bytes: Vec<u8> = Vec::with_capacity(n);
        self.file.seek(seek)?;
        self.file.read_exact(&mut bytes)?;

        Ok(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_BPS_PATH: &str = "test-data/Grand_Poo_World_3.bps";

    #[test]
    fn test_header_check() {
        let mut file = File::open(TEST_BPS_PATH).unwrap();
        let check = BpsHeader::magic_check(&mut file);

        assert!(check.is_ok());
    }

    #[test]
    fn test_header_content() {
        let mut file = File::open(TEST_BPS_PATH).unwrap();
        let header = BpsHeader::try_from(&mut file).unwrap();

        println!("{header:?}");
    }
}
