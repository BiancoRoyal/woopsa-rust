use serde::{Deserialize, Serialize};

use crate::protocol::element::Element;

use crate::protocol::element::WoopsaElement;
use crate::protocol::method_argument_info::WoopsaMethodArgumentInfo;
use crate::protocol::value::WoopsaValue;
use crate::protocol::value_type::WoopsaValueType;

use crate::protocol::struct_type::WoopsaStructType;

use std::time::SystemTime;

pub trait Method {
    fn arguments(&self) -> Vec<WoopsaMethodArgumentInfo>;
    fn return_type(&self) -> WoopsaValueType;
    fn invoke() -> WoopsaValue;
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WoopsaMethod {
    name: String,
    arguments: Vec<WoopsaMethodArgumentInfo>,
    return_type: WoopsaValueType,
}

impl WoopsaMethod {

    pub fn type_of(&self) -> &'static str {
        return "WoopsaMethod";
    }

    pub fn get_struct_type(&self) -> WoopsaStructType {
        return WoopsaStructType::WoopsaMethod;
    }

    pub fn get_name(&self) -> String {
        return self.name.clone();
    }

    pub fn get_arguments(&self) ->  Vec<WoopsaMethodArgumentInfo> {
        return self.arguments.clone();
    }

    pub fn get_return_type(&self) ->  WoopsaValueType {
        return self.return_type.clone();
    }

    pub fn as_element(&self) -> WoopsaElement {
        return WoopsaElement::new(self.get_name());
    }
}

impl Element for WoopsaMethod {
    fn name(&self) -> String {
        return self.get_name();
    }
}

impl Method for WoopsaMethod {
    fn arguments(&self)  -> Vec<WoopsaMethodArgumentInfo> {
        return self.get_arguments();
    }

    fn return_type(&self)  -> WoopsaValueType {
        return self.get_return_type();
    }

    fn invoke() -> WoopsaValue {
        return WoopsaValue::new(String::from("Null"), WoopsaValueType::Null);
    }
}
