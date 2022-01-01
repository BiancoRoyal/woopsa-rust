use serde::{Deserialize, Serialize};

use crate::protocol::element::Element;
use crate::protocol::method_argument_info::WoopsaMethodArgumentInfo;
use crate::protocol::value::WoopsaValue;
use crate::protocol::value_type::WoopsaValueType;
use std::time::SystemTime;

pub trait Method {
    fn arguments(&self) -> Vec<WoopsaMethodArgumentInfo>;
    fn return_type(&self) -> WoopsaValueType;
    fn invoke() -> WoopsaValue;
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WoopsaMethod {
    pub name: String,
    pub arguments: Vec<WoopsaMethodArgumentInfo>,
    pub return_type: WoopsaValueType,
}

impl WoopsaMethod {
    fn type_of(&self) -> &'static str {
        "WoopsaMethod"
    }

    fn get_name(&self) -> String {
        return self.name.clone();
    }

    fn get_arguments(&self) ->  Vec<WoopsaMethodArgumentInfo> {
        return self.arguments;
    }
    fn get_return_type(&self) ->  WoopsaValueType {
        return self.return_type.clone();
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
        return WoopsaValue {
            as_text: String::from("Null"),
            timestamp: SystemTime::now(),
            value_type: WoopsaValueType::Null,
        };
    }
}
