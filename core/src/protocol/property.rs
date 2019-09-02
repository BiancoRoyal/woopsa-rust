use crate::protocol::element::Element;
use crate::protocol::element::WoopsaElement;
use crate::protocol::value::WoopsaValue;
use crate::protocol::value_type::WoopsaValueType;

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
