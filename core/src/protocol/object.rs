use crate::protocol::container::WoopsaContainer;
use crate::protocol::method::WoopsaMethod;
use crate::protocol::property::WoopsaProperty;
use crate::protocol::value::WoopsaValue;
use crate::protocol::value_type::WoopsaValueType;

pub trait Object {
}

pub struct WoopsaObject {
    pub container: WoopsaContainer,
    pub properties: Vec<WoopsaProperty>,
    pub methods: Vec<WoopsaMethod>,
}

impl WoopsaObject {
}

impl Object for WoopsaObject {
}
