use serde::{Deserialize, Serialize};

use crate::protocol::element::WoopsaElement;
use crate::protocol::method_argument_info::WoopsaMethodArgumentInfo;
use crate::protocol::value::WoopsaValue;
use crate::protocol::value_type::WoopsaValueType;
use std::time::SystemTime;

pub trait Method {
    fn type_of(&self) -> &'static str;
    fn name(&self) -> String;
    fn invoke() -> WoopsaValue;
}

#[derive(Serialize, Deserialize)]
pub struct WoopsaMethod {
    pub element: WoopsaElement,
    pub arguments: Vec<WoopsaMethodArgumentInfo>,
    pub return_type: WoopsaValueType,
}

impl WoopsaMethod {}

impl Method for WoopsaMethod {
    fn type_of(&self) -> &'static str {
        "WoopsaMethod"
    }

    fn name(&self) -> String {
        return self.element.name.clone();
    }

    fn invoke() -> WoopsaValue {
        return WoopsaValue {
            as_text: String::from("Null"),
            timestamp: SystemTime::now(),
            value_type: WoopsaValueType::Null,
        };
    }
}
