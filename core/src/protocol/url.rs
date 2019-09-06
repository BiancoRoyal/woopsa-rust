use serde::{Deserialize, Serialize};

use crate::protocol::struct_type::WoopsaStructType;

pub trait URL {
    fn type_of(&self) -> WoopsaStructType;
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WoopsaURL {
    pub protocol: String,
    pub server_address: String,
    pub route_prefix: String,
    pub woopsa_verb: String,
    pub woopsa_path: String,
    pub content_type: String,
}

impl WoopsaURL {
    pub fn type_of(&self) -> WoopsaStructType {
        WoopsaStructType::WoopsaURL
    }
}

impl URL for WoopsaURL {
    fn type_of(&self) -> WoopsaStructType {
        self.type_of()
    }
}
