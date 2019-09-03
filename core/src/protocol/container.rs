use crate::protocol::element::WoopsaElement;

use std::collections::HashMap;

trait Container {
    fn type_of(&self) -> &'static str;
    fn clear(&mut self); 
}

pub struct WoopsaContainer {
    pub element: WoopsaElement,
    pub items: HashMap<String, WoopsaContainer>
}

impl WoopsaContainer {
    pub fn name(&self) -> String {
        return self.element.name.clone();
    }

    pub fn insert_item(&mut self, item: WoopsaContainer) {
        self.items.insert(item.element.name.clone(), item);
    }

    pub fn get_item(&mut self, name: String) {
        self.items.get(&name);
    }

    pub fn remove_item(&mut self, item: WoopsaContainer) {
        self.items.remove(&(item.element.name));
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }
}

impl Container for WoopsaContainer {
    fn type_of(&self) -> &'static str {
        "WoopsaContainer"
    }

    fn clear(&mut self) {
        self.clear();
    }
}
