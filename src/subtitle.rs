use crate::timestamp::Timestamp;
use alloc::string::String;
use core::fmt::Display;

#[derive(Clone)]
pub struct Subtitle {
    index: usize,
    start: Timestamp,
    end: Timestamp,
    pub text: String,
}

impl Subtitle {
    pub fn new(index: usize, start: Timestamp, end: Timestamp, text: String) -> Self {
        Self {
            index,
            start,
            end,
            text,
        }
    }
}

impl Display for Subtitle {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}\n{} --> {}\n{}\n",
            self.index, self.start, self.end, self.text
        )
    }
}
