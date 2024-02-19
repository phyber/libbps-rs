// Mechanics for the target (output) file.
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
pub struct TargetFile {
    file: File,
}

impl TargetFile {
    pub fn new<P>(path: &P) -> Result<Self, Errors>
    where
        P: AsRef<Path> + ?Sized,
    {
        let file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(path)?;

        let target = Self {
            file,
        };

        Ok(target)
    }

    pub fn seek(&mut self, seek: SeekFrom) -> Result<(), Errors> {
        self.file.seek(seek)?;

        Ok(())
    }

    pub fn write(&mut self, buf: &[u8]) -> Result<(), Errors> {
        self.file.write(buf)?;

        Ok(())
    }
}
