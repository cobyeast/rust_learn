// Exercism Rust - Clock
// Date: Oct 30, 2020
// Status: Complete

use std::fmt;

#[derive(Eq, PartialEq, Debug)]
pub struct Clock {
  hours: i32,
  minutes: i32,
}

impl fmt::Display for Clock {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:02}:{:02}", self.hours, self.minutes)
  }
}

impl Clock {
  pub fn new(hours: i32, minutes: i32) -> Self {
    Clock::make(hours, minutes)
  }
  fn make(hours: i32, minutes: i32) -> Self {
    let mut mins = (hours * 60) + minutes;

    mins = mins.rem_euclid(1440);

    Clock {
      hours: (mins / 60) % 24,
      minutes: mins % 60,
    }
  }
  pub fn add_minutes(&mut self, minutes: i32) -> Self {
    self.minutes += minutes;
    Clock::make(self.hours, self.minutes)
  }
}
