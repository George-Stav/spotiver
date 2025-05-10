use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
enum Reason {
    Market, Product, Explicit, None
}

/* Spotify API object */
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RestrictionsObject {
    reason: Reason
}

impl Default for RestrictionsObject {
    fn default() -> Self {
	RestrictionsObject { reason: Reason::None }
    }
}
