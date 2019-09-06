use serde::{Deserialize, Serialize};

use crate::protocol::element::WoopsaElement;
use crate::protocol::method_argument_info::WoopsaMethodArgumentInfo;
use crate::protocol::struct_type::WoopsaStructType;
use crate::protocol::value::WoopsaValue;
use crate::protocol::value_type::WoopsaValueType;

use std::collections::HashMap;
use std::time::SystemTime;

pub trait Method {
    fn type_of(&self) -> WoopsaStructType;
    fn name(&self) -> String;
    fn invoke() -> WoopsaValue;
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WoopsaMethod {
    pub element: WoopsaElement,
    pub arguments: HashMap<String, WoopsaMethodArgumentInfo>,
    pub return_type: WoopsaValueType,
}

impl WoopsaMethod {
    pub fn new(element: WoopsaElement, return_type: WoopsaValueType) -> WoopsaMethod {
        WoopsaMethod {
            element: element,
            arguments: HashMap::new(),
            return_type: return_type,
        }
    }

    pub fn add_argument(&mut self, argument: WoopsaMethodArgumentInfo) {
        self.arguments.insert(argument.name(), argument);
    }

    pub fn remove_argument(&mut self, argument: WoopsaMethodArgumentInfo) {
        self.arguments.remove(&(argument.name));
    }

    pub fn clear_arguments(&mut self) {
        self.arguments.clear();
    }

    pub fn type_of(&self) -> WoopsaStructType {
        WoopsaStructType::WoopsaMethod
    }
}

impl Method for WoopsaMethod {
    fn type_of(&self) -> WoopsaStructType {
        self.type_of()
    }

    fn name(&self) -> String {
        self.element.name.clone()
    }

    fn invoke() -> WoopsaValue {
        WoopsaValue {
            as_text: String::from("Null"),
            timestamp: SystemTime::now(),
            value_type: WoopsaValueType::Null,
        }
    }
}
