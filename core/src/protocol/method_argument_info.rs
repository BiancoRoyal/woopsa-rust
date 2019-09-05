use serde::{Deserialize, Serialize};

use crate::protocol::value_type::WoopsaValueType;

pub trait MethodArgumentInfo {}

#[derive(Serialize, Deserialize)]
pub struct WoopsaMethodArgumentInfo {
    pub name: String,
    pub value_type: WoopsaValueType,
}

impl MethodArgumentInfo {}

impl MethodArgumentInfo for WoopsaMethodArgumentInfo {}
