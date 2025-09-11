use serde::{Serialize, Deserialize};
use serde_json::Number;

use std::fmt;

/* My Implementation of serde_json::Number.
 * Makes it so the default trait can be derived automatically. */
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SjNumber(Number);

impl Default for SjNumber {
    fn default() -> Self {
	SjNumber(Number::from(0))
    }
}

impl fmt::Display for SjNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
