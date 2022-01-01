use serde::{Deserialize, Serialize};

use crate::protocol::element::Element;
use crate::protocol::value::WoopsaValue;
use crate::protocol::value_type::WoopsaValueType;

use std::fmt;
use std::time::SystemTime;

pub trait Property {
    fn value(&self) -> WoopsaValue;
    fn value_type(&self) -> WoopsaValueType;
    fn is_read_only(&self) -> bool;
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WoopsaProperty {
    pub name: String,
    pub value: WoopsaValue,
    pub value_type: WoopsaValueType,
    pub readonly: bool,
}

impl WoopsaProperty {
    pub fn new_readonly(element_name: String, value: String, value_type: WoopsaValueType) -> WoopsaProperty {
        WoopsaProperty {
            name: element_name,
            value: WoopsaValue {
                as_text: value,
                timestamp: SystemTime::now(),
                value_type
            },
            value_type,
            readonly: true
        }
    }

    pub fn new(element_name: String, value: String, value_type: WoopsaValueType) -> WoopsaProperty {
        WoopsaProperty {
            name: element_name,
            value: WoopsaValue {
                as_text: value,
                timestamp: SystemTime::now(),
                value_type
            },
            value_type,
            readonly: false
        }
    }

    pub fn get_name(&self) -> String {
        return self.name.clone()
    }

    pub fn set_value(&mut self, value: String, value_type: WoopsaValueType) {
        self.value.value_type = value_type;
        self.value.as_text = value;
        self.value.timestamp = SystemTime::now();
    }

    pub fn get_value(&self) -> WoopsaValue {
        return self.value
    }

    pub fn set_value_type(&mut self, value_type: WoopsaValueType) {
        self.value_type = value_type;
    }

    pub fn get_value_type(&self) -> WoopsaValueType {
        self.value_type
    }

    pub fn set_readonly(&mut self) {
        self.readonly = true;
    }

    pub fn get_readonly(&self) -> bool {
        return self.readonly;
    }

    pub fn set_read_write(&mut self) {
        self.readonly = false;
    }

    fn type_of(&self) -> &'static str {
        "WoopsaProperty"
    }
}

impl Element for WoopsaProperty {
    fn name(&self) -> String {
        return self.get_name();
    }
}

impl Property for WoopsaProperty {
    fn value(&self) -> WoopsaValue {
        return self.get_value();
    }

    fn value_type(&self) -> WoopsaValueType{
       return self.get_value_type();
    }

    fn is_read_only(&self) -> bool {
        return self.get_readonly();
    }
}

impl fmt::Display for WoopsaProperty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} with value {})", 
        self.name,
        self.value.as_text)
    }
}
