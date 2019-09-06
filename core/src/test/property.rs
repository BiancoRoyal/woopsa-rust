#[cfg(test)]
mod unit_test {
    use crate::protocol::property::WoopsaProperty;
    use crate::protocol::value_type::WoopsaValueType;
    use crate::protocol::struct_type::WoopsaStructType;

    #[test]
    fn it_should_get_woopsaproperty_with_name() {
        let property = WoopsaProperty::new(String::from("Temperature"), String::from("24.3"), WoopsaValueType::Real);
        assert_eq!(property.name(), String::from("Temperature"));
    }

    #[test]
    fn it_should_get_woopsaproperty_type_of() {
        let property = WoopsaProperty::new(String::from("Temperature"), String::from("24.3"), WoopsaValueType::Real);
        assert_eq!(property.type_of(), WoopsaStructType::WoopsaProperty);
    }
}