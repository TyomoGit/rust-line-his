#![warn(clippy::pedantic)]

pub mod history;
pub mod line_content;
pub mod processing;

#[cfg(feature = "calendar")]
pub mod calendar;
