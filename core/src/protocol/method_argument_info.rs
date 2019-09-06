use serde::{Deserialize, Serialize};

use crate::protocol::struct_type::WoopsaStructType;
use crate::protocol::value_type::WoopsaValueType;

pub trait MethodArgumentInfo {
    fn type_of(&self) -> WoopsaStructType;
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WoopsaMethodArgumentInfo {
    pub name: String,
    pub value_type: WoopsaValueType,
}

impl WoopsaMethodArgumentInfo {
    pub fn new(argument_name: String, value_type: WoopsaValueType) -> WoopsaMethodArgumentInfo {
        WoopsaMethodArgumentInfo {
            name: argument_name,
            value_type: value_type,
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn has_type(&self) -> WoopsaValueType {
        self.value_type.clone()
    }

    pub fn type_of(&self) -> WoopsaStructType {
        WoopsaStructType::WoopsaMethodArgumentInfo
    }
}

impl MethodArgumentInfo for WoopsaMethodArgumentInfo {
    fn type_of(&self) -> WoopsaStructType {
        self.type_of()
    }
}
