use crate::core::value_type::WoopsaValueType;

pub trait MethodArgumentInfo {}

pub struct WoopsaMethodArgumentInfo {
    pub name: String,
    pub value_type: WoopsaValueType,
}

impl MethodArgumentInfo {}

impl MethodArgumentInfo for WoopsaMethodArgumentInfo {}
