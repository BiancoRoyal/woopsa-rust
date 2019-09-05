use serde::{Deserialize, Serialize};

use crate::protocol::element::WoopsaElement;
use crate::protocol::container::WoopsaContainer;
use crate::protocol::method::WoopsaMethod;
use crate::protocol::property::WoopsaProperty;

use std::collections::HashMap;
use std::fmt;

pub trait Object {
    fn type_of(&self) -> &'static str;
}

#[derive(Serialize, Deserialize)]
pub struct WoopsaObject {
    pub container: WoopsaContainer,
    pub properties: HashMap<String, WoopsaProperty>,
    pub methods: HashMap<String, WoopsaMethod>,
}

impl WoopsaObject {
    pub fn new(element_name: String) -> WoopsaObject {
        WoopsaObject {
            container: WoopsaContainer {
                element: WoopsaElement {
                    name: element_name,
                },
                items: HashMap::new(),
            },
            properties: HashMap::new(),
            methods: HashMap::new(),
        }
    }

    pub fn name(&self) -> String {
        return self.container.element.name.clone();
    }

    pub fn register_container(&mut self, container: WoopsaContainer) {
        self.container = container;
    }

    pub fn insert_item(&mut self, item: WoopsaObject) {
        self.container.insert_item(item);
    }

    pub fn remove_item(&mut self, item: WoopsaObject) {
        self.container.remove_item(item);
    }

    pub fn add_property(&mut self, property: WoopsaProperty) {
        self.properties.insert(property.element.name.clone(), property);
    }

    pub fn remove_property(&mut self, property: WoopsaProperty) {
        self.properties.remove(&(property.element.name));
    }

    pub fn add_method(&mut self, method: WoopsaMethod) {
        self.methods.insert(method.element.name.clone(), method);
    }

    pub fn remove_method(&mut self, method: WoopsaMethod) {
        self.methods.remove(&(method.element.name));
    }

    pub fn find_item_by_name(&self, name: String) -> &WoopsaObject {
        return self.container.items.get(&name).unwrap();
    }

    pub fn find_property_by_name(&self, name: String) -> &WoopsaProperty {
        return self.properties.get(&name).unwrap();
    }

    pub fn find_method_by_name(&self, name: String) -> &WoopsaMethod {
        return self.methods.get(&name).unwrap();
    }

    pub fn clear(&mut self) {
        self.container.clear();
        self.properties.clear();
        self.methods.clear();
    }
}

impl Object for WoopsaObject {
    fn type_of(&self) -> &'static str {
        "WoopsaObject"
    }
}

impl fmt::Display for WoopsaObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} with items count {}, properties count {}, and methods count {})", 
        self.container.element.name,
        self.container.items.len(),
        self.properties.len(),
        self.methods.len())
    }
}
