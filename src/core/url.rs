trait URL {}

pub struct WoopsaURL {
    pub protocol: String,
    pub server_address: String,
    pub route_prefix: String,
    pub woopsa_verb: String,
    pub woopsa_path: String,
    pub content_type: String,
}

impl WoopsaURL {}

impl URL for WoopsaURL {}
