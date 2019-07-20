trait Container {}

pub struct WoopsaContainer {
    pub items: Vec<WoopsaContainer>,
}

impl WoopsaContainer {}

impl Container for WoopsaContainer {}
