use crate::protocol::container::WoopsaContainer;
use crate::protocol::element::WoopsaElement;
use crate::protocol::method::WoopsaMethod;
use crate::protocol::property::WoopsaProperty;

use std::collections::HashMap;

pub trait Object {
    fn type_of(&self) -> &'static str;
}

pub struct WoopsaObject {
    pub element: WoopsaElement,
    pub container: WoopsaContainer,
    pub properties: HashMap<String, WoopsaProperty>,
    pub methods: HashMap<String, WoopsaMethod>,
}

impl WoopsaObject {
    pub fn name(&self) -> String {
        return self.element.name.clone();
    }

    pub fn register_container(&mut self, container: WoopsaContainer) {
        self.container = container;
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
