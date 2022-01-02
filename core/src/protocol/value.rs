use serde::{Deserialize, Serialize};

use crate::protocol::value_type::WoopsaValueType;

use crate::protocol::struct_type::WoopsaStructType;

use std::time::SystemTime;

pub trait Value {
    fn as_text(&self) -> String;
    fn time_stamp(&self) -> SystemTime;
    fn value_type(&self) -> WoopsaValueType;
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WoopsaValue {
    as_text: String,
    timestamp: SystemTime,
    value_type: WoopsaValueType,
}

impl WoopsaValue {

    pub fn type_of(&self) -> &'static str {
        return "WoopsaValue";
    }

    pub fn get_struct_type(&self) -> WoopsaStructType {
        return WoopsaStructType::WoopsaValue;
    }

    pub fn new(as_text: String, value_type: WoopsaValueType) -> WoopsaValue {
       WoopsaValue {
            as_text,
            timestamp: SystemTime::now(),
            value_type
        }
    }

    pub fn new_with_timestamp(as_text: String, timestamp: SystemTime, value_type: WoopsaValueType) -> WoopsaValue {
       WoopsaValue {
            as_text,
            timestamp,
            value_type
        }
    }
    pub fn as_text(&self) -> String {
        return self.as_text.clone();
    }

    pub fn get_time_stamp(&self) -> SystemTime {
        return self.timestamp.clone();
    }

    pub fn get_value_type(&self) -> WoopsaValueType {
        return self.value_type.clone();
    }

    pub fn check_as_text_by_value_type(&self) {
        // todo: check if as_text and value_type are to be compatible
    }

    pub fn eq(&self, value: WoopsaValue) -> bool {
        return self.as_text == value.as_text 
        && self.timestamp == value.timestamp
        && self.value_type == value.value_type;
    }
}

impl Value for WoopsaValue {
    fn as_text(&self) -> String {
        return self.as_text();
    }

    fn time_stamp(&self) -> SystemTime {
        return self.get_time_stamp();
    }

    fn value_type(&self) -> WoopsaValueType {
        return self.get_value_type();
    }
}
