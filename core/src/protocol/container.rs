use serde::{Deserialize, Serialize};

use crate::protocol::constant::*;
use crate::protocol::element::Element;

use crate::protocol::element::WoopsaElement;

use std::collections::HashMap;
use std::fmt;

pub trait Container {
    fn items(&self) -> HashMap<String, WoopsaContainer>;
}

pub trait WoopsaContainerService {
    fn get_name(&self) -> String;
    fn get_path(&self) -> String;
    // Items
    fn insert_item(&mut self, item: WoopsaContainer);
    fn remove_item(&mut self, item: WoopsaContainer);
    fn get_items(&self) -> HashMap<String, WoopsaContainer>;
    fn set_items(&mut self, items: HashMap<String, WoopsaContainer>);
    fn get_item_path(&self, item_name: String) -> String;
    fn get_item_by_path(&self, item_path: String) -> &WoopsaContainer;
    fn find_item_by_name(&self, name: String) -> &WoopsaContainer;
    fn clear_items(&mut self);
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WoopsaContainer {
    container_name: String,
    item_list: HashMap<String, WoopsaContainer>
}

impl WoopsaContainer {

    pub fn type_of(&self) -> &'static str {
        return "WoopsaContainer"
    }

    pub fn new(element_name: String) -> WoopsaContainer {
        WoopsaContainer {
            container_name: element_name,
            item_list: HashMap::new(),
        }
    }

    pub fn new_with_items(element_name: String, item_list: HashMap<String, WoopsaContainer>) -> WoopsaContainer {
        WoopsaContainer {
            container_name: element_name,
            item_list,
        }
    }

    pub fn get_name(&self) -> String {
        return self.container_name.clone();
    }

    pub fn is_root(&self) -> bool {
        return self.container_name == WOOPSA_ROOT_ELEMENT_NAME
    }

    pub fn is_not_root(&self) -> bool {
        return self.container_name != WOOPSA_ROOT_ELEMENT_NAME
    }

    pub fn clear(&mut self) {
        self.item_list.clear();
    }

    pub fn insert_item(&mut self, item: WoopsaContainer) {
        self.item_list.insert(item.name(), item);
    }

    pub fn remove_item(&mut self, item: WoopsaContainer) {
        self.item_list.remove(&(item.container_name));
    }

    pub fn get_items(&self) -> HashMap<String, WoopsaContainer> {
        return self.item_list.clone();
    }

    pub fn set_items(&mut self, items: HashMap<String, WoopsaContainer>) {
        return self.item_list = items;
    }

    pub fn clear_items(&mut self) {
        self.item_list.clear();
    }

    pub fn find_item_by_name(&self, name: String) -> &WoopsaContainer {
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

    pub fn get_item_by_path(&self, item_path: String) -> &WoopsaContainer {
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

    pub fn as_element(&self) -> WoopsaElement {
        return WoopsaElement::new(self.get_name());
    }
}

impl WoopsaContainerService for WoopsaContainer {

    fn get_name(&self) -> String {
        return self.get_name();
    }

    fn insert_item(&mut self, item: WoopsaContainer) {
        self.insert_item(item);
    }

    fn remove_item(&mut self, item: WoopsaContainer) {
        self.remove_item(item);
    }

    fn get_items(&self) -> HashMap<String, WoopsaContainer> {
        return self.get_items();
    }

    fn set_items(&mut self, items: HashMap<String, WoopsaContainer>) {
        self.set_items(items);
    }

    fn clear_items(&mut self) {
        self.clear_items();
    }

    fn find_item_by_name(&self, name: String) -> &WoopsaContainer {
        return self.find_item_by_name(name);
    }

    fn get_path(&self) -> String {
        return self.get_path();
    }

    fn get_item_path(&self, item_name: String) -> String {
       return self.get_item_path(item_name);
    }

    fn get_item_by_path(&self, item_path: String) -> &WoopsaContainer {
        return self.get_item_by_path(item_path);
    }
}

impl Element for WoopsaContainer {
    fn name(&self) -> String {
        return self.get_name()
    }
}

impl Container for WoopsaContainer {
    fn items(&self) -> HashMap<String, WoopsaContainer> {
        return self.get_items().clone()
    }
}

impl fmt::Display for WoopsaContainer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(container {} with items count {})", 
        self.name(),
        self.get_items().len())
    }
}
