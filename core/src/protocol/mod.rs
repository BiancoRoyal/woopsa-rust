pub mod constant;
pub mod container;
pub mod element;
pub mod error_type;
pub mod link;
pub mod method;
pub mod method_argument_info;
pub mod object;
pub mod property;
pub mod url;
pub mod value;
pub mod value_type;
pub mod verb;

#[cfg(test)]
mod tests {
    use crate::protocol::container::WoopsaContainer;
    use crate::protocol::element::WoopsaElement;
    use crate::protocol::object::WoopsaObject;
    use std::collections::HashMap;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn it_build_simple_model() {
        let mut root = WoopsaObject {
            container: WoopsaContainer {
                element: WoopsaElement {
                    name: String::from("Root"),
                },
                items: HashMap::new(),
            },
            properties: HashMap::new(),
            methods: HashMap::new(),
        };

        let objects = WoopsaObject {
            container: WoopsaContainer {
                element: WoopsaElement {
                    name: String::from("Objects"),
                },
                items: HashMap::new(),
            },
            properties: HashMap::new(),
            methods: HashMap::new(),
        };

        root.container.insert_item(objects);

        let objects = root.find_item_by_name(String::from("Objects"));
        assert_eq!(objects.name(), String::from("Objects"));
    }
}
