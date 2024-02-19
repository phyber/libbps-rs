// Patching routine that will be the main entry point for most users.
use crate::action::Action;
use crate::bps::Bps;
use crate::source::SourceFile;
use crate::target::TargetFile;
use crate::errors::Errors;
use std::io::{
    prelude::*,
    SeekFrom,
};
use std::fs::{
    File,
    OpenOptions,
};
use std::path::Path;

#[derive(Debug)]
pub struct Patcher {
    bps: Bps,
    source_file: SourceFile,
    target_file: TargetFile,
}

impl Patcher {
    pub fn new<P: AsRef<Path> + ?Sized>(
        bps_file: &P,
        source_path: &P,
        target_path: &P,
    ) -> Result<Self, Errors> {
        let bps = Bps::new(bps_file)?;
        let source_file = SourceFile::new(source_path)?;
        let target_file = TargetFile::new(target_path)?;

        let patcher = Self {
            bps,
            source_file,
            target_file,
        };

        Ok(patcher)
    }

    pub fn patch(&mut self) -> Result<(), Errors> {
        let mut original_offset = 0;
        let mut output_offset = 0;
        let mut patch_offset = 0;

        // Loop over the patch, performing the given instructions.
        while patch_offset < (self.bps.patch_size() - 12) {
            match self.bps.action()? {
                Action::SourceRead(mut len) => {
                    self.target_file.seek(SeekFrom::Start(output_offset))?;

                    // Can probably read all bytes at once for `len`.
                    while len > 0 {
                        let bytes = self.source_file.read(1)?;
                        self.target_file.write(&bytes)?;

                        len -= 1;
                    }
                },
                Action::TargetRead(mut len) => {
                    while len > 0 {
                        len -= 1;
                    }

                    patch_offset += 1;
                },
                Action::SourceCopy(len) => {
                },
                Action::TargetCopy(len) => {
                },
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

}
