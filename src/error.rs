use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct TatamiError {
    details: String,
}

impl TatamiError {
    pub fn new(msg: &str) -> TatamiError {
        TatamiError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for TatamiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for TatamiError {
    fn description(&self) -> &str {
        &self.details
    }
}
