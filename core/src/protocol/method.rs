use crate::protocol::method_argument_info::WoopsaMethodArgumentInfo;
use crate::protocol::value::WoopsaValue;
use crate::protocol::value_type::WoopsaValueType;

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
        unimplemented!()
    }
}
