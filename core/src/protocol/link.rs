use serde::{Deserialize, Serialize};

pub trait Link {
    fn url(&self) -> String;
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WoopsaLink {
    pub server_url: String,
    pub resource_url: String,
    pub woopsa_path: String,
    pub absolute_path: String,
    pub relative_path: String
}

impl WoopsaLink {
    fn type_of(&self) -> &'static str {
        "WoopsaLink"
    }

    fn get_server_url(&self) -> String {
        return self.server_url.clone();
    }
}

impl Link for WoopsaLink {
    fn url(&self) -> String {
        return self.get_server_url()
    }
}
