use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub enum WoopsaValueType {
    Null,
    Logical,
    Integer,
    Real,
    DateTime,
    TimeSpan,
    Text,
    WoopsaLink,
    JsonData,
    ResourceUrl,
}
