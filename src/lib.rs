#![forbid(unsafe_code)]
#![no_std]

extern crate alloc;

mod parser;
mod subtitle;
mod timestamp;

#[cfg(test)]
mod tests;

pub use parser::parse;
pub use subtitle::Subtitle;
pub use timestamp::Timestamp;
