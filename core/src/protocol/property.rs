use crate::protocol::element::WoopsaElement;
use crate::protocol::value::WoopsaValue;
use crate::protocol::value_type::WoopsaValueType;

use std::fmt;
use std::time::SystemTime;

pub trait Property {
    fn type_of(&self) -> &'static str;
    fn name(&self) -> String;
    fn is_read_only(&self) -> bool;
}

pub struct WoopsaProperty {
    pub element: WoopsaElement,
    pub value: WoopsaValue,
    pub value_type: WoopsaValueType,
    pub readonly: bool,
}

impl WoopsaProperty {
    pub fn new_readonly(element_name: String, value: String, value_type: WoopsaValueType) -> WoopsaProperty {
        WoopsaProperty {
            element: WoopsaElement {
                name: element_name,
            },
            value: WoopsaValue {
                as_text: value,
                timestamp: SystemTime::now(),
                value_type: value_type
            },
            value_type: value_type,
            readonly: true
        }
    }

    pub fn new(element_name: String, value: String, value_type: WoopsaValueType) -> WoopsaProperty {
        WoopsaProperty {
            element: WoopsaElement {
                name: element_name,
            },
            value: WoopsaValue {
                as_text: value,
                timestamp: SystemTime::now(),
                value_type: value_type
            },
            value_type: value_type,
            readonly: false
        }
    }

    pub fn set_value(&mut self, value: String, value_type: WoopsaValueType) {
        self.value.value_type = value_type;
        self.value.as_text = value;
        self.value.timestamp = SystemTime::now();
    }

    pub fn set_readonly(&mut self) {
        self.readonly = true;
    }

    pub fn set_read_write(&mut self) {
        self.readonly = false;
    }
}

impl Property for WoopsaProperty {
    fn type_of(&self) -> &'static str {
        "WoopsaProperty"
    }

    fn name(&self) -> String {
        return self.element.name.clone();
    }
    
    fn is_read_only(&self) -> bool {
        return self.readonly.clone();
    }
}

impl fmt::Display for WoopsaProperty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} with value {})", 
        self.element.name,
        self.value.as_text)
    }
}
