use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct PreAuthorizedCode {
    pub user_pin_required: bool,
    pub pre_authorized_code: String,
}
