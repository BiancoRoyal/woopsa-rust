use serde::{Deserialize, Serialize};

use crate::protocol::constant::*;
use crate::protocol::container::WoopsaContainer;
use crate::protocol::element::WoopsaElement;
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
    is_root_object: bool,
}

impl WoopsaObject {
    pub fn new(element_name: String) -> WoopsaObject {
        WoopsaObject {
            container: WoopsaContainer {
                element: WoopsaElement { name: element_name },
                items: HashMap::new(),
            },
            properties: HashMap::new(),
            methods: HashMap::new(),
            is_root_object: false,
        }
    }

    pub fn root() -> WoopsaObject {
        WoopsaObject {
            container: WoopsaContainer {
                element: WoopsaElement {
                    name: String::from(WOOPSA_ROOT_ELEMENT_NAME),
                },
                items: HashMap::new(),
            },
            properties: HashMap::new(),
            methods: HashMap::new(),
            is_root_object: true,
        }
    }

    pub fn is_root(&self) -> bool {
        self.is_root_object
    }

    pub fn is_not_root(&self) -> bool {
        !self.is_root_object
    }

    pub fn name(&self) -> String {
        return self.container.element.name.clone();
    }

    pub fn set_name(&mut self, name: String) {
        self.container.element.name = name;
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
        self.properties
            .insert(property.element.name.clone(), property);
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

    pub fn get_path(&self) -> String {
        let mut path = String::new();
        if self.is_not_root() {
            path.push(WOOPSA_PATH_SEPARATOR);
            path.push_str(self.name().as_str());
        } else {
            path.push(WOOPSA_ROOT_PATH);
        }
        path
    }

    pub fn get_item_path(&self, item_name: String) -> String {
        let mut path = String::new();
        if self.is_not_root() {
            path.push(WOOPSA_PATH_SEPARATOR);
            path.push_str(self.name().as_str());
            path.push(WOOPSA_PATH_SEPARATOR);
            path.push_str(self.find_item_by_name(item_name).name().as_str());
        } else {
            if WOOPSA_ROOT_PATH.eq_ignore_ascii_case(&WOOPSA_PATH_SEPARATOR) {
                path.push(WOOPSA_ROOT_PATH);
            } else {
                path.push(WOOPSA_ROOT_PATH);
                path.push(WOOPSA_PATH_SEPARATOR);
            }
            path.push_str(self.find_item_by_name(item_name).name().as_str());
        }
        path
    }

    pub fn get_item_by_path(&self, item_path: String) -> &WoopsaObject {
        let mut name = String::new();

        if item_path.contains(WOOPSA_PATH_SEPARATOR) {
            let mut names: Vec<&str> = item_path.split(WOOPSA_PATH_SEPARATOR).collect();
            names.reverse();
            let first_path = names.pop().unwrap();
            println!("first_path: {}", first_path.clone());
            if first_path.is_empty() {
                name.push(WOOPSA_PATH_SEPARATOR);
                name.push_str(names.pop().unwrap());
            } else {
                name.push_str(first_path);
            }
        } else {
            name.push_str(item_path.as_str());
        }
        let next_item_path = item_path.replace(name.as_str(), "");
        println!(
            "{} name: {} next is {}",
            self.name(),
            name.clone(),
            next_item_path.clone()
        );

        if name.contains(WOOPSA_PATH_SEPARATOR) || next_item_path.contains(WOOPSA_PATH_SEPARATOR) {
            let next_group_item = self.find_item_by_name(name.replace(WOOPSA_PATH_SEPARATOR, ""));
            if next_item_path.is_empty() {
                println!("simple find next group item {}", name);
                next_group_item
            } else {
                println!("go depper for hierarchy {}", next_item_path.clone());
                next_group_item.get_item_by_path(next_item_path)
            }
        } else {
            println!("simple find item {}", name);
            self.find_item_by_name(name)
        }
    }
}

impl Object for WoopsaObject {
    fn type_of(&self) -> &'static str {
        "WoopsaObject"
    }
}

impl fmt::Display for WoopsaObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(object named {} with items count {}, properties count {}, and methods count {})",
            self.name(),
            self.container.items.len(),
            self.properties.len(),
            self.methods.len()
        )
    }
}

#[test]
fn it_should_get_woopsaobject_root_path() {
    let root = WoopsaObject::root();
    assert_eq!(root.get_path(), "/");
}

#[test]
fn it_should_get_woopsaobject_path() {
    let objects = WoopsaObject::new(String::from("Objects"));
    assert_eq!(objects.get_path(), "/Objects");
}

#[test]
fn it_should_get_woopsaobject_path_in_hierarchy() {
    let mut root = WoopsaObject::root();
    let mut objects = WoopsaObject::new(String::from("Objects"));
    objects.insert_item(WoopsaObject::new(String::from("WeatherStation")));
    root.insert_item(objects);
    let objects = root.find_item_by_name(String::from("Objects"));
    let weather_station = objects.find_item_by_name(String::from("WeatherStation"));
    assert_eq!(weather_station.get_path(), "/WeatherStation");
}

#[test]
fn it_should_get_woopsaobject_item_path_in_hierarchy() {
    let mut root = WoopsaObject::root();
    let mut objects = WoopsaObject::new(String::from("Objects"));
    objects.insert_item(WoopsaObject::new(String::from("WeatherStation")));
    root.insert_item(objects);
    let objects = root.find_item_by_name(String::from("Objects"));
    assert_eq!(
        objects.get_item_path(String::from("WeatherStation")),
        "/Objects/WeatherStation"
    );
}

#[test]
fn it_should_get_woopsaobject_item_path_from_root() {
    let mut root = WoopsaObject::root();
    let mut objects = WoopsaObject::new(String::from("Objects"));
    objects.insert_item(WoopsaObject::new(String::from("WeatherStation")));
    root.insert_item(objects);
    assert_eq!(root.get_item_path(String::from("Objects")), "/Objects");
}

#[test]
fn it_should_get_woopsaobject_item_by_root_path_from_hierarchy() {
    let mut root = WoopsaObject::root();
    let mut objects = WoopsaObject::new(String::from("Objects"));
    let mut weatherstation = WoopsaObject::new(String::from("WeatherStation"));
    weatherstation.insert_item(WoopsaObject::new(String::from("Thermostat")));
    objects.insert_item(weatherstation);
    root.insert_item(objects);
    assert_eq!(
        root.get_item_by_path(String::from("/Objects/WeatherStation/Thermostat"))
            .name(),
        String::from("Thermostat")
    );
}

#[test]
fn it_should_get_woopsaobject_item_by_path_from_hierarchy() {
    let mut root = WoopsaObject::root();
    let mut objects = WoopsaObject::new(String::from("Objects"));
    let mut weatherstation = WoopsaObject::new(String::from("WeatherStation"));
    weatherstation.insert_item(WoopsaObject::new(String::from("Thermostat")));
    objects.insert_item(weatherstation);
    root.insert_item(objects);

    let objects = root.find_item_by_name(String::from("Objects"));
    assert_eq!(
        objects
            .get_item_by_path(String::from("WeatherStation/Thermostat"))
            .name(),
        String::from("Thermostat")
    );
}
