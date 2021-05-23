use std::fmt;
use std::fmt::Formatter;
use std::error::Error;

pub enum RuntimeMode {
    Continuous,
    Single,
}

pub const MINUTES_MULTIPLIER: u64 = 60;

#[derive(Debug)]
pub struct IllegalArgumentError {
    details: String,
}

impl IllegalArgumentError {
    pub fn new(details: &str) -> IllegalArgumentError {
        IllegalArgumentError { details: String::from(details) }
    }
}

impl fmt::Display for IllegalArgumentError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "An illegal argument was encountered. Reason: {}", self.details)
    }
}

impl Error for IllegalArgumentError {}

#[derive(Debug)]
pub struct RuntimeError {
    details: String,
}

impl RuntimeError {
    pub fn new(details: &str) -> RuntimeError {
        RuntimeError { details: String::from(details) }
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "An error was encountered during runtime. Reason: {}", self.details)
    }
}

impl Error for RuntimeError {}
