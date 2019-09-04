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
    use crate::protocol::object::WoopsaObject;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn it_build_simple_model() {
        let mut root = WoopsaObject::new(String::from("Root"));
        root.container.insert_item(WoopsaObject::new(String::from("Server")));
        
        let mut objects = WoopsaObject::new(String::from("Objects"));
        objects.container.insert_item(WoopsaObject::new(String::from("Devices")));
        objects.container.insert_item(WoopsaObject::new(String::from("Models")));

        println!("/root/ is {}", &root);
        for (key, value) in &(root.container.items) {
            println!("item of root -> {}: {}", key, value);
        }

        println!("/root/objects/ is {}", &objects);
        for (key, value) in &(objects.container.items) {
            println!("item of objects -> {}: {}", key, value);
        }

        assert_eq!(objects.name(), String::from("Objects"));
    }
}
