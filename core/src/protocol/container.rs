use serde::{Deserialize, Serialize};

use crate::protocol::element::Element;
use crate::protocol::object::WoopsaObject;
// use crate::protocol::constant::*;

use std::collections::HashMap;
use std::fmt;

pub trait Container {
    fn items(&self) -> HashMap<String, WoopsaObject>;
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WoopsaContainer {
    pub name: String,
    pub items: HashMap<String, WoopsaObject>
}

impl WoopsaContainer {
    fn type_of(&self) -> &'static str {
        "WoopsaContainer"
    }

    pub fn get_name(&self) -> String {
        return self.name.clone();
    }

    pub fn insert_item(&mut self, item: WoopsaObject) {
        self.items.insert(item.name(), item);
    }

    pub fn remove_item(&mut self, item: WoopsaObject) {
        self.items.remove(&(item.name));
    }

    fn get_items(&mut self) -> HashMap<String, WoopsaObject> {
        return self.items;
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }
}

impl Element for WoopsaContainer {
    fn name(&self) -> String { self.get_name() }
}

impl Container for WoopsaContainer {
    fn items(&self) -> HashMap<String, WoopsaObject> {
        return self.get_items()
    }
}

impl fmt::Display for WoopsaContainer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(container {} with items count {})", 
        self.name(),
        self.get_items().len())
    }
}
