use serde::{Deserialize, Serialize};

use crate::protocol::value_type::WoopsaValueType;

pub trait MethodArgumentInfo {}

#[derive(Serialize, Deserialize, Clone)]
pub struct WoopsaMethodArgumentInfo {
    argument_name: String,
    value_type: WoopsaValueType,
}

impl WoopsaMethodArgumentInfo {
    fn type_of(&self) -> &'static str {
        "WoopsaMethodArgumentInfo"
    }
}

impl MethodArgumentInfo for WoopsaMethodArgumentInfo {}
