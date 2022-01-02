use serde::{Deserialize, Serialize};

use crate::protocol::element::Element;

use crate::protocol::element::WoopsaElement;
use crate::protocol::value::WoopsaValue;
use crate::protocol::value_type::WoopsaValueType;

use crate::protocol::struct_type::WoopsaStructType;

use std::fmt;
use std::time::SystemTime;

pub trait Property {
    fn value(&self) -> WoopsaValue;
    fn value_type(&self) -> WoopsaValueType;
    fn is_read_only(&self) -> bool;
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WoopsaProperty {
    name: String,
    value: WoopsaValue,
    value_type: WoopsaValueType,
    readonly: bool,
}

impl WoopsaProperty {

    pub fn type_of(&self) -> &'static str {
        return "WoopsaProperty"
    }

    pub fn get_struct_type(&self) -> WoopsaStructType {
        return WoopsaStructType::WoopsaProperty;
    }

    pub fn new_readonly(element_name: String, value: String, value_type: WoopsaValueType) -> WoopsaProperty {
        WoopsaProperty {
            name: element_name,
            value: WoopsaValue::new(value, value_type),
            value_type,
            readonly: true
        }
    }

    pub fn new(element_name: String, value: String, value_type: WoopsaValueType) -> WoopsaProperty {
        WoopsaProperty {
            name: element_name,
            value: WoopsaValue::new(value, value_type),
            value_type,
            readonly: false
        }
    }

    pub fn get_name(&self) -> String {
        return self.name.clone()
    }

    pub fn set_value(&mut self, value: String, value_type: WoopsaValueType) {
        self.set_value_type(value_type);
        self.value = WoopsaValue::new(value, value_type);
    }

    pub fn get_value(&self) -> WoopsaValue {
        return self.value.clone()
    }

    pub fn set_value_type(&mut self, value_type: WoopsaValueType) {
        self.value_type = value_type;
    }

    pub fn get_value_type(&self) -> WoopsaValueType {
        return self.value_type
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

    pub fn as_element(&self) -> WoopsaElement {
        return WoopsaElement::new(self.get_name());
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
        self.name(),
        self.value().as_text())
    }
}
