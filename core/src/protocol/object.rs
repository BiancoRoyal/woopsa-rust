use serde::{Deserialize, Serialize};

use crate::protocol::constant::*;

use crate::protocol::element::Element;
use crate::protocol::container::Container;

use crate::protocol::element::WoopsaElement;
use crate::protocol::container::WoopsaContainer;
use crate::protocol::container::WoopsaContainerService;
use crate::protocol::method::WoopsaMethod;
use crate::protocol::property::WoopsaProperty;

use crate::protocol::struct_type::WoopsaStructType;

use std::collections::HashMap;
use std::fmt;

pub trait Object {
    fn is_root(&self) -> bool;
    fn properties(&self) -> HashMap<String, WoopsaProperty>;
    fn methods(&self) -> HashMap<String, WoopsaMethod>;
}

pub trait WoopsaObjectService {
    fn get_name(&self) -> String;
    fn get_path(&self) -> String;
    // Items
    fn insert_item(&mut self, item: WoopsaObject);
    fn remove_item(&mut self, item: WoopsaObject);
    fn get_items(&self) -> HashMap<String, WoopsaObject>;
    fn get_item_path(&self, item_name: String) -> String;
    fn get_item_by_path(&self, item_path: String) -> &WoopsaObject;
    fn find_item_by_name(&self, name: String) -> &WoopsaObject;
    fn clear_items(&mut self);
    // Root
    fn is_not_root(&self) -> bool;
    // Properties
    fn add_property(&mut self, property: WoopsaProperty);
    fn remove_property(&mut self, property: WoopsaProperty);
    fn get_properties(&self) -> HashMap<String, WoopsaProperty>;
    fn find_property_by_name(&self, name: String) -> &WoopsaProperty;
    // Methods
    fn add_method(&mut self, method: WoopsaMethod);
    fn remove_method(&mut self, method: WoopsaMethod);
    fn get_methods(&self) -> HashMap<String, WoopsaMethod>;
    fn find_method_by_name(&self, name: String) -> &WoopsaMethod;
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WoopsaObject {
    object_name: String,
    item_list: HashMap<String, WoopsaObject>,
    property_list: HashMap<String, WoopsaProperty>,
    method_list: HashMap<String, WoopsaMethod>,
    is_root_object: bool,
}

impl WoopsaObject {

    pub fn type_of(&self) -> &'static str {
        return "WoopsaObject"
    }

    pub fn get_struct_type(&self) -> WoopsaStructType {
        return WoopsaStructType::WoopsaObject;
    }

    pub fn new(element_name: String) -> WoopsaObject {
        WoopsaObject {
            object_name: element_name,
            item_list: HashMap::new(),
            property_list: HashMap::new(),
            method_list: HashMap::new(),
            is_root_object: false,
        }
    }

    pub fn root() -> WoopsaObject {
        WoopsaObject {
            object_name:  String::from(WOOPSA_ROOT_ELEMENT_NAME),
            item_list: HashMap::new(),
            property_list: HashMap::new(),
            method_list: HashMap::new(),
            is_root_object: true,
        }
    }

    pub fn get_name(&self) -> String {
        return self.object_name.clone();
    }

    pub fn set_name(&mut self, name: String) {
        self.object_name = name;
    }

    pub fn clear(&mut self) {
        self.item_list.clear();
        self.property_list.clear();
        self.method_list.clear();
    }

    pub fn is_not_root(&self) -> bool {
        return !self.is_root_object
    }

    pub fn add_property(&mut self, property: WoopsaProperty) {
        self.property_list.insert(property.name(), property);
    }

    pub fn remove_property(&mut self, property: WoopsaProperty) {
        self.property_list.remove(&(property.name()));
    }

    pub fn get_properties(&self) -> HashMap<String, WoopsaProperty> {
        return self.property_list.clone();
    }

    pub fn find_property_by_name(&self, name: String) -> &WoopsaProperty {
        return self.property_list.get(&name).unwrap();
    }

    pub fn add_method(&mut self, method: WoopsaMethod) {
        self.method_list.insert(method.name(), method);
    }

    pub fn remove_method(&mut self, method: WoopsaMethod) {
        self.method_list.remove(&(method.name()));
    }

    pub fn get_methods(&self) -> HashMap<String, WoopsaMethod> {
        return self.method_list.clone();
    }

    pub fn find_method_by_name(&self, name: String) -> &WoopsaMethod {
        return self.method_list.get(&name).unwrap();
    }

    pub fn insert_item(&mut self, item: WoopsaObject) {
        self.item_list.insert(item.name(), item);
    }

    pub fn remove_item(&mut self, item: WoopsaObject) {
        self.item_list.remove(&(item.object_name));
    }

    pub fn get_items(&self) -> HashMap<String, WoopsaObject> {
        return self.item_list.clone();
    }

    pub fn clear_items(&mut self) {
        self.item_list.clear();
    }

    pub fn find_item_by_name(&self, name: String) -> &WoopsaObject {
        return self.item_list.get(&name).unwrap();
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
        if name.contains(WOOPSA_PATH_SEPARATOR) || next_item_path.contains(WOOPSA_PATH_SEPARATOR) {
            let next_group_item = self.find_item_by_name(name.replace(WOOPSA_PATH_SEPARATOR, ""));
            if next_item_path.is_empty() {
                next_group_item
            } else {
                next_group_item.get_item_by_path(next_item_path)
            }
        } else {
            self.find_item_by_name(name)
        }
    }

    pub fn convert_items_to_container(&self, items: HashMap<String, WoopsaObject>) -> HashMap<String, WoopsaContainer>{

        let mut container_item_list = HashMap::new();

        for (name, item) in items.iter() {
            let mut container = WoopsaContainer::new(name.to_string());
            let child_items = self.convert_items_to_container(item.get_items());
            container.set_items(child_items.clone());
            container_item_list.insert(name.to_string(), container);
        }

        return container_item_list
    }

    pub fn as_element(&self) -> WoopsaElement {
        return WoopsaElement::new(self.get_name());
    }

    pub fn as_container(&self) -> WoopsaContainer {
        return WoopsaContainer::new_with_items(self.get_name(), self.convert_items_to_container(self.get_items()));
    }
}

impl Element for WoopsaObject {
    fn name(&self) -> String {
        return self.get_name()
    }
}

impl Container for WoopsaObject {
    fn items(&self) -> HashMap<String, WoopsaContainer> {
        let mut container_item_list = HashMap::new();

        for (name, item) in self.get_items().iter() {
            let mut container = WoopsaContainer::new(name.to_string());
            let child_items = self.convert_items_to_container(item.get_items());
            container.set_items(child_items);
            container_item_list.insert(name.to_string(), container);
        }

        return container_item_list;
    }
}

impl Object for WoopsaObject {
    fn is_root(&self) -> bool {
        return self.is_root_object
    }

    fn properties(&self) -> HashMap<String, WoopsaProperty> {
        return self.get_properties()
    }

    fn methods(&self) -> HashMap<String, WoopsaMethod> {
        return self.get_methods()
    }
}

impl WoopsaObjectService for WoopsaObject {

    fn is_not_root(&self) -> bool {
        return self.is_not_root()
    }

    fn add_property(&mut self, property: WoopsaProperty) {
        self.add_property(property);
    }

    fn remove_property(&mut self, property: WoopsaProperty) {
        self.remove_property(property);
    }

    fn get_properties(&self) -> HashMap<String, WoopsaProperty> {
        return self.get_properties();
    }

    fn find_property_by_name(&self, name: String) -> &WoopsaProperty {
        return self.find_property_by_name(name);
    }

    fn add_method(&mut self, method: WoopsaMethod) {
        self.add_method(method);
    }

    fn remove_method(&mut self, method: WoopsaMethod) {
        self.remove_method(method);
    }

    fn get_methods(&self) -> HashMap<String, WoopsaMethod> {
        return self.get_methods();
    }

    fn find_method_by_name(&self, name: String) -> &WoopsaMethod {
        return self.find_method_by_name(name);
    }

    fn get_name(&self) -> String {
        return self.get_name();
    }

    fn insert_item(&mut self, item: WoopsaObject) {
        self.insert_item(item);
    }

    fn remove_item(&mut self, item: WoopsaObject) {
        self.remove_item(item);
    }

    fn get_items(&self) -> HashMap<String, WoopsaObject> {
        return self.get_items();
    }

    fn clear_items(&mut self) {
        self.clear_items();
    }

    fn find_item_by_name(&self, name: String) -> &WoopsaObject {
        return self.find_item_by_name(name);
    }

    fn get_path(&self) -> String {
        return self.get_path();
    }

    fn get_item_path(&self, item_name: String) -> String {
        return self.get_item_path(item_name);
    }

    fn get_item_by_path(&self, item_path: String) -> &WoopsaObject {
        return self.get_item_by_path(item_path);
    }
}

impl fmt::Display for WoopsaObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(object named {} with items count {}, properties count {}, and methods count {})",
            self.name(),
            self.items().len(),
            self.properties().len(),
            self.methods().len()
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
