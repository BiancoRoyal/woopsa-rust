use serde::{Deserialize, Serialize};

use crate::protocol::element::WoopsaElement;
use crate::protocol::object::WoopsaObject;
use crate::protocol::struct_type::WoopsaStructType;

use std::collections::HashMap;
use std::fmt;

trait Container {
    fn type_of(&self) -> WoopsaStructType;
    fn clear(&mut self);
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WoopsaContainer {
    pub element: WoopsaElement,
    pub items: HashMap<String, WoopsaObject>,
}

impl WoopsaContainer {
    pub fn new(element_name: String) -> WoopsaContainer {
        WoopsaContainer {
            element: WoopsaElement { name: element_name },
            items: HashMap::new(),
        }
    }
    pub fn name(&self) -> String {
        self.element.name.clone()
    }

    pub fn insert_item(&mut self, item: WoopsaObject) {
        self.items.insert(item.name().clone(), item);
    }

    pub fn remove_item(&mut self, item: WoopsaObject) {
        self.items.remove(&(item.container.element.name));
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn type_of(&self) -> WoopsaStructType {
        WoopsaStructType::WoopsaContainer
    }
}

impl Container for WoopsaContainer {
    fn type_of(&self) -> WoopsaStructType {
        self.type_of()
    }

    fn clear(&mut self) {
        self.clear();
    }
}

impl fmt::Display for WoopsaContainer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(container {} with items count {})",
            self.name(),
            self.items.len()
        )
    }
}
