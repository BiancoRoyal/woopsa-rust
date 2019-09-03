trait Link {}

pub struct WoopsaLink {
    pub server_url: String,
    pub resource_url: String,
    pub woopsa_path: String,
    pub absolute_path: String,
    pub relative_path: String
}

impl WoopsaLink {}

impl Link for WoopsaLink {}
