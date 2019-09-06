use serde::{Deserialize, Serialize};

use crate::protocol::struct_type::WoopsaStructType;
use crate::protocol::value_type::WoopsaValueType;

use std::time::SystemTime;

pub trait Value {
    fn type_of(&self) -> WoopsaStructType;
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WoopsaValue {
    pub as_text: String,
    pub timestamp: SystemTime,
    pub value_type: WoopsaValueType,
}

impl WoopsaValue {
    pub fn new(as_text: String, value_type: WoopsaValueType) -> WoopsaValue {
        WoopsaValue {
            as_text: as_text,
            timestamp: SystemTime::now(),
            value_type: value_type
        }
    }

    pub fn check_as_text_by_value_type(&self) {
        // todo: check if as_text and value_type are to be compatible
    }

    pub fn text(&self) -> String {
        self.as_text.clone()
    }

    pub fn get_value(&self) -> WoopsaValue {
        WoopsaValue {
            as_text: self.as_text.clone(),
            timestamp: SystemTime::now(),
            value_type: self.value_type.clone()
        }
    }

    pub fn eq(&self, value: WoopsaValue) -> bool {
        return self.as_text == value.as_text
            && self.timestamp == value.timestamp
            && self.value_type == value.value_type;
    }

    pub fn type_of(&self) -> WoopsaStructType {
        WoopsaStructType::WoopsaValue
    }
}

impl Value for WoopsaValue {
    fn type_of(&self) -> WoopsaStructType {
       self.type_of()
    }
}
