use serde::{Deserialize, Serialize};

use crate::protocol::struct_type::WoopsaStructType;

pub trait Link {
    fn url(&self) -> String;
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WoopsaLink {
    pub server_url: String,
    pub resource_url: String,
    pub woopsa_path: String,
    pub absolute_path: String,
    pub relative_path: String
}

impl WoopsaLink {
    pub fn type_of(&self) -> &'static str {
        return "WoopsaLink";
    }

    pub fn get_struct_type(&self) -> WoopsaStructType {
        return WoopsaStructType::WoopsaLink;
    }

    pub fn get_server_url(&self) -> String {
        return self.server_url.clone();
    }
}

impl Link for WoopsaLink {
    fn url(&self) -> String {
        return self.get_server_url()
    }
}
