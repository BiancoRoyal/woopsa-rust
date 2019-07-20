use crate::core::method::WoopsaMethod;
use crate::core::property::WoopsaProperty;
use crate::core::value::WoopsaValue;
use crate::core::value_type::WoopsaValueType;
use std::time::SystemTime;

pub trait Object {
    fn invoke(&self) -> WoopsaValue;
}

pub struct WoopsaObject {
    pub properties: Vec<WoopsaProperty>,
    pub methods: Vec<WoopsaMethod>,
}

impl WoopsaObject {}

impl Object for WoopsaObject {
    fn invoke(&self) -> WoopsaValue {
        return WoopsaValue {
            as_text: String::from("Null"),
            timestamp: SystemTime::now(),
            value_type: WoopsaValueType::Null,
        };
    }
}
