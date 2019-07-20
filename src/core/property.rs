use crate::core::element::Element;
use crate::core::element::WoopsaElement;
use crate::core::value::WoopsaValue;
use crate::core::value_type::WoopsaValueType;

pub trait Property {
    fn is_read_only(&self) -> bool;
}

pub struct WoopsaProperty {
    pub element: WoopsaElement,
    pub value: WoopsaValue,
    pub value_type: WoopsaValueType,
    pub readonly: bool,
}

impl WoopsaProperty {}

impl Property for WoopsaProperty {
    fn is_read_only(&self) -> bool {
        return self.readonly;
    }
}
