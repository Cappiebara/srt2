use core::fmt::Display;

#[derive(Clone)]
pub struct Timestamp {
    hours: usize,
    minutes: usize,
    seconds: usize,
    miseconds: usize,
}

impl Timestamp {
    pub fn new(hours: usize, minutes: usize, seconds: usize, miseconds: usize) -> Self {
        Self {
            hours,
            minutes,
            seconds,
            miseconds,
        }
    }
}

impl Display for Timestamp {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{:02}:{:02}:{:02},{:03}",
            self.hours, self.minutes, self.seconds, self.miseconds
        )
    }
}
