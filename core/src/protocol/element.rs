use serde::{Deserialize, Serialize};

pub trait Element {
    fn type_of(&self) -> &'static str;
}

#[derive(Serialize, Deserialize)]
pub struct WoopsaElement {
    pub name: String,
}

impl WoopsaElement {
}

impl Element for WoopsaElement {
    fn type_of(&self) -> &'static str {
        "WoopsaElement"
    }
}
