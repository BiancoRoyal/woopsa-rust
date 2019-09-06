#[cfg(test)]
mod unit_test {
    use crate::protocol::value::WoopsaValue;
    use crate::protocol::value_type::WoopsaValueType;
    use crate::protocol::struct_type::WoopsaStructType;

    #[test]
    fn it_should_get_woopsavalue_with_name() {
        let value = WoopsaValue::new(String::from("24.3"), WoopsaValueType::Real);
        assert_eq!(value.text(), String::from("24.3"));
    }

    #[test]
    fn it_should_get_woopsavalue_type_of() {
        let value = WoopsaValue::new(String::from("24.3"), WoopsaValueType::Real);
        assert_eq!(value.type_of(), WoopsaStructType::WoopsaValue);
    }
}