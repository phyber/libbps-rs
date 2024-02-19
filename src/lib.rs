// libbps-rs
//
// Crate for patching SNES ROMs.
pub mod action;
pub mod bps;
pub mod decode;
pub mod errors;
pub mod patcher;
pub mod source;
pub mod target;

pub use action::Action;
pub use bps::Bps;
pub use errors::Errors;
pub use patcher::Patcher;
pub use source::SourceFile;
pub use target::TargetFile;

#[cfg(test)]
mod tests {
    use super::*;

}
