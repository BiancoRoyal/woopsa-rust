#[cfg(test)]
mod unit_test {
    use crate::protocol::constant::*;
    use crate::protocol::container::WoopsaContainer;
    use crate::protocol::struct_type::WoopsaStructType;

    #[test]
    fn it_should_get_woopsacontainer_with_element_name() {
        let root = WoopsaContainer::new(String::from(WOOPSA_ROOT_ELEMENT_NAME));
        assert_eq!(root.items.len(), 0);
        assert_eq!(root.name(), String::from(WOOPSA_ROOT_ELEMENT_NAME));
    }

    #[test]
    fn it_should_get_woopsacontainer_type_of() {
        let root = WoopsaContainer::new(String::from(WOOPSA_ROOT_ELEMENT_NAME));
        assert_eq!(root.type_of(), WoopsaStructType::WoopsaContainer);
    }
}
