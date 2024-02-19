// Mechanics for the target (output) file.
use crate::errors::Errors;
use std::io::{
    prelude::*,
    SeekFrom,
};
use std::fs::File;
use std::path::Path;

#[derive(Debug)]
pub struct SourceFile {
    file: File,
}

impl SourceFile {
    pub fn new<P>(path: &P) -> Result<Self, Errors>
    where
        P: AsRef<Path> + ?Sized,
    {
        let file = File::open(path)?;

        let target = Self {
            file,
        };

        Ok(target)
    }

    pub fn read(&mut self, n: usize) -> Result<Vec<u8>, Errors> {
        let mut buf: Vec<u8> = vec![0; n];
        self.file.read_exact(&mut buf)?;

        Ok(buf)
    }
}
