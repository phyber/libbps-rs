// Errors
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Errors {
    #[error("bad bps file")]
    BadBps,

    #[error("bad crc32")]
    BadCrc32,

    #[error("bad crc32 for bps file")]
    BadCrc32Bps,

    #[error("bad crc32 for source file")]
    BadCrc32Source,

    #[error("bad crc32 for target file")]
    BadCrc32Target,

    #[error("bad header")]
    BadHeader,

    #[error("invalid read size")]
    InvalidReadSize,

    #[error(transparent)]
    Io(#[from] std::io::Error),
}
