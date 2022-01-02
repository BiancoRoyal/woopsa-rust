
#[cfg(test)]
mod unit_test {
    use crate::protocol::constant::*;
    use crate::protocol::element::WoopsaElement;
    use crate::protocol::struct_type::WoopsaStructType;

    #[test]
    fn it_should_get_woopsaelement_with_name() {
        let root_element = WoopsaElement { name: String::from(WOOPSA_ROOT_ELEMENT_NAME) };
        assert_eq!(root_element.name(), String::from(WOOPSA_ROOT_ELEMENT_NAME));
    }

    #[test]
    fn it_should_get_woopsaelement_type_of() {
        let root_element = WoopsaElement { name: String::from(WOOPSA_ROOT_ELEMENT_NAME) };
        assert_eq!(root_element.type_of(), WoopsaStructType::WoopsaElement);
    }
}