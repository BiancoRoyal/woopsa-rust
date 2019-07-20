trait Link {}

pub struct WoopsaLink {
    pub server_url: String,
    pub woopsa_path: String,
}

impl WoopsaLink {}

impl Link for WoopsaLink {}
