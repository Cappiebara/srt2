use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq)]
pub struct Subtitle {
    pub index: usize,
    pub start: u32,
    pub end: u32,
    pub text: String,
}

impl Subtitle {
    pub fn new(index: usize, start: u32, end: u32, text: String) -> Self {
        Self {
            index,
            start,
            end,
            text,
        }
    }
}

impl Display for Subtitle {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let timestamp = |time: u32| {
            let ms = time & 0x3FF;
            let s = (time >> 10) & 0x3F;
            let m = (time >> 16) & 0x3F;
            let h = (time >> 22) & 0xF;

            format!("{:02}:{:02}:{:02},{:03}", h, m, s, ms)
        };

        write!(
            f,
            "{}\n{} --> {}\n{}\n",
            self.index,
            timestamp(self.start),
            timestamp(self.end),
            self.text
        )
    }
}
