use crate::core::value_type::WoopsaValueType;
use std::time::SystemTime;

pub trait Value {}

pub struct WoopsaValue {
    pub as_text: String,
    pub timestamp: SystemTime,
    pub value_type: WoopsaValueType,
}

impl WoopsaValue {}

impl Value for WoopsaValue {}
