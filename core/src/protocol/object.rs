use crate::protocol::container::WoopsaContainer;
use crate::protocol::method::WoopsaMethod;
use crate::protocol::property::WoopsaProperty;
use crate::protocol::value::WoopsaValue;
use crate::protocol::value_type::WoopsaValueType;
use std::time::SystemTime;

pub trait Object {
    fn invoke(&self) -> WoopsaValue;
}

pub struct WoopsaObject {
    container: WoopsaContainer,
    pub properties: Vec<WoopsaProperty>,
    pub methods: Vec<WoopsaMethod>,
}

impl WoopsaObject {
}

impl Object for WoopsaObject {
    fn invoke(&self) -> WoopsaValue {
        return WoopsaValue {
            as_text: String::from("Null"),
            timestamp: SystemTime::now(),
            value_type: WoopsaValueType::Null,
        };
    }
}
