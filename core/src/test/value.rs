#[cfg(test)]
mod unit_test {
    use crate::protocol::struct_type::WoopsaStructType;
    use crate::protocol::value::WoopsaValue;
    use crate::protocol::value_type::WoopsaValueType;

    #[test]
    fn it_should_get_woopsavalue_text() {
        let value = WoopsaValue::new(String::from("24.3"), WoopsaValueType::Real);
        assert_eq!(value.text(), String::from("24.3"));
    }

    #[test]
    fn it_should_get_woopsavalue_as_str() {
        let value = WoopsaValue::new(String::from("24.3"), WoopsaValueType::Real);
        assert_eq!(value.as_str(), String::from("24.3").as_str());
    }

    #[test]
    fn it_should_get_woopsavalue_type_of() {
        let value = WoopsaValue::new(String::from("24.3"), WoopsaValueType::Real);
        assert_eq!(value.type_of(), WoopsaStructType::WoopsaValue);
    }

    #[test]
    fn it_should_equal_woopsavalue_with_timestamp_false() {
        let value_a = WoopsaValue::new(String::from("24.3"), WoopsaValueType::Real);
        let value_b = WoopsaValue::new(String::from("24.3"), WoopsaValueType::Real);
        assert_eq!(value_a.eq(value_b), false);
    }

    #[test]
    fn it_should_equal_woopsavalue_with_itself() {
        let value_a = WoopsaValue::new(String::from("24.3"), WoopsaValueType::Real);
        assert_eq!(value_a.eq(value_a.clone()), true);
    }

    #[test]
    fn it_should_get_woopsavalue_typed_real() {
        let value = WoopsaValue::new(String::from("24.3"), WoopsaValueType::Real);
        assert_eq!(value.as_f32().unwrap(), "24.3".parse::<f32>().unwrap());
    }

    #[test]
    fn it_should_get_woopsavalue_typed_integer() {
        let value = WoopsaValue::new(String::from("24"), WoopsaValueType::Integer);
        assert_eq!(value.as_i32().unwrap(), "24".parse::<i32>().unwrap());
    }

    #[test]
    fn it_should_get_woopsavalue_typed_real_64() {
        let value = WoopsaValue::new(String::from("24.3"), WoopsaValueType::Real);
        assert_eq!(value.as_f64().unwrap(), "24.3".parse::<f64>().unwrap());
    }

    #[test]
    fn it_should_get_woopsavalue_typed_integer_64() {
        let value = WoopsaValue::new(String::from("24"), WoopsaValueType::Integer);
        assert_eq!(value.as_i64().unwrap(), "24".parse::<i64>().unwrap());
    }

    #[test]
    fn it_should_get_woopsavalue_typed_logical_true() {
        let value = WoopsaValue::new(String::from("true"), WoopsaValueType::Logical);
        assert_eq!(value.as_logical().unwrap(), true);
    }

    #[test]
    fn it_should_get_woopsavalue_typed_logical_false() {
        let value = WoopsaValue::new(String::from("false"), WoopsaValueType::Logical);
        assert_eq!(value.as_logical().unwrap(), false);
    }

    #[test]
    fn it_should_get_woopsavalue_empty_typed_logical_false() {
        let value = WoopsaValue::new(String::from(""), WoopsaValueType::Logical);
        assert_eq!(value.as_logical().unwrap(), false);
    }

    #[test]
    fn it_should_get_woopsavalue_as_text() {
        let value = WoopsaValue::new(String::from("Hello world!"), WoopsaValueType::Text);
        assert_eq!(value.as_text().unwrap(), String::from("Hello world!"));
    }
}
