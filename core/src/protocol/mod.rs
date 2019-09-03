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
    use std::collections::HashMap;
    use crate::protocol::element::WoopsaElement;
    use crate::protocol::container::WoopsaContainer;
    use crate::protocol::object::WoopsaObject;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test] 
    fn it_build_simple_model() {
        let root_container = WoopsaContainer { 
            element: WoopsaElement { 
                name: String::from("Root") 
            },
            items: HashMap::new()
        };

        let objects_container = WoopsaContainer { 
            element: WoopsaElement { 
                name: String::from("Objcts") 
            },
            items: HashMap::new()
        };

        let mut root = WoopsaObject {
            element: WoopsaElement { 
                    name: String::from("Root") 
            },
            container: root_container,
            properties: HashMap::new(),
            methods: HashMap::new()
        };

        let objects = WoopsaObject {
            element: WoopsaElement { 
                name: String::from("Objects") 
            }, 
            container: objects_container,
            properties: HashMap::new(),
            methods: HashMap::new()
        };

        root.container.insert_item(objects.container);
    }
}
