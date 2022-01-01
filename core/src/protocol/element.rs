use serde::{Deserialize, Serialize};

pub trait Element {
    fn name(&self) -> String;
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WoopsaElement {
    pub name: String,
}

impl WoopsaElement {
    fn type_of(&mut self) -> &'static str {
        "WoopsaElement"
    }

    fn get_name(&mut self) -> String {
        self.name.clone()
    }
}

impl Element for WoopsaElement {
    fn name(&self) -> String { self.get_name() }
}
