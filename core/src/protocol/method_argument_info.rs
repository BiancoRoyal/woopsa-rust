use serde::{Deserialize, Serialize};

use crate::protocol::value_type::WoopsaValueType;

use crate::protocol::struct_type::WoopsaStructType;

pub trait MethodArgumentInfo {}

#[derive(Serialize, Deserialize, Clone)]
pub struct WoopsaMethodArgumentInfo {
    argument_name: String,
    value_type: WoopsaValueType,
}

impl WoopsaMethodArgumentInfo {
    fn type_of(&self) -> &'static str {
        return "WoopsaMethodArgumentInfo";
    }

    pub fn get_struct_type(&self) -> WoopsaStructType {
        WoopsaStructType::WoopsaMethod
    }
}

impl MethodArgumentInfo for WoopsaMethodArgumentInfo {}
