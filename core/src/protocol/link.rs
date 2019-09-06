use serde::{Deserialize, Serialize};

use crate::protocol::struct_type::WoopsaStructType;

pub trait Link {
    fn type_of(&self) -> WoopsaStructType;
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WoopsaLink {
    pub server_url: String,
    pub resource_url: String,
    pub woopsa_path: String,
    pub absolute_path: String,
    pub relative_path: String,
}

impl WoopsaLink {
    pub fn type_of(&self) -> WoopsaStructType {
        WoopsaStructType::WoopsaLink
    }
}

impl Link for WoopsaLink {
    fn type_of(&self) -> WoopsaStructType {
        self.type_of()
    }
}
