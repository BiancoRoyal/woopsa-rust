use serde::{Deserialize, Serialize};

use crate::protocol::struct_type::WoopsaStructType;

pub trait Element {
    fn type_of(&self) -> WoopsaStructType;
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WoopsaElement {
    pub name: String,
}

impl WoopsaElement {
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn type_of(&self) -> WoopsaStructType {
        WoopsaStructType::WoopsaElement
    }
}

impl Element for WoopsaElement {
    fn type_of(&self) -> WoopsaStructType {
        self.type_of()
    }
}
