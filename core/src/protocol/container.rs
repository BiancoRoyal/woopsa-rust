use crate::protocol::element::WoopsaElement;

trait Container {
}

pub struct WoopsaContainer {
    pub element: WoopsaElement,
    pub items: Vec<WoopsaContainer>,
}

impl WoopsaContainer {}

impl Container for WoopsaContainer {
}
