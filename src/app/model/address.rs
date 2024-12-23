use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Validate, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct Address {
    pub country: String,
    pub line1: String,
    pub line2: Option<String>,
    pub city: String,
    pub state: String,
    pub zipcode: String,
}

impl Address {
    pub fn new(
        country: String,
        line1: String,
        line2: Option<String>,
        city: String,
        state: String,
        zipcode: String,
    ) -> Address {
        Address {
            country,
            line1,
            line2,
            city,
            state,
            zipcode,
        }
    }
}
