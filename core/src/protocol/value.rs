use crate::protocol::value_type::WoopsaValueType;
use std::time::SystemTime;

pub trait Value {
}

pub struct WoopsaValue {
    pub as_text: String,
    pub timestamp: SystemTime,
    pub value_type: WoopsaValueType,
}

impl WoopsaValue {
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
}
