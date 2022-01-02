use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub enum WoopsaStructType {
    WoopsaElement,
    WoopsaContainer,
    WoopsaObject,
    WoopsaProperty,
    WoopsaMethod,
    WoopsaMethodArgumentInfo,
    WoopsaValue,
    WoopsaLink,
    WoopsaURL,
}
