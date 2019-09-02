use crate::protocol::method_argument_info::WoopsaMethodArgumentInfo;
use crate::protocol::value::WoopsaValue;
use crate::protocol::value_type::WoopsaValueType;
use std::time::SystemTime;

pub trait Method {
    fn invoke() -> WoopsaValue;
}

pub struct WoopsaMethod {
    pub arguments: Vec<WoopsaMethodArgumentInfo>,
    pub return_type: WoopsaValueType,
}

impl WoopsaMethod {}

impl Method for WoopsaMethod {
    fn invoke() -> WoopsaValue {
        return WoopsaValue {
            as_text: String::from("Null"),
            timestamp: SystemTime::now(),
            value_type: WoopsaValueType::Null,
        };
    }
}
