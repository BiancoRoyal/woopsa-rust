use crate::protocol::element::WoopsaElement;
use crate::protocol::object::WoopsaObject;

use std::collections::HashMap;

trait Container {
    fn type_of(&self) -> &'static str;
    fn clear(&mut self); 
}

pub struct WoopsaContainer {
    pub element: WoopsaElement,
    pub items: HashMap<String, WoopsaObject>
}

impl WoopsaContainer {
    pub fn name(&self) -> String {
        return self.element.name.clone();
    }

    pub fn insert_item(&mut self, item: WoopsaObject) {
        self.items.insert(item.container.element.name.clone(), item);
    }

    pub fn remove_item(&mut self, item: WoopsaObject) {
        self.items.remove(&(item.container.element.name));
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
