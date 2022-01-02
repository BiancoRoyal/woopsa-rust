use serde::{Deserialize, Serialize};

pub trait Element {
    fn name(&self) -> String;
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WoopsaElement {
    element_name: String,
}

impl WoopsaElement {

    pub fn type_of(&self) -> &'static str {
        return "WoopsaElement";
    }

    pub fn new(element_name: String) -> WoopsaElement {
        WoopsaElement {
            element_name
        }
    }

    pub fn get_name(&self) -> String {
        return self.element_name.clone();
    }
}

impl Element for WoopsaElement {
    fn name(&self) -> String {
        return self.get_name()
    }
}
