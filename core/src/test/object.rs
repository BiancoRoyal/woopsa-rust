#[cfg(test)]
mod unit_test {
    use crate::protocol::object::WoopsaObject;
    use crate::protocol::property::WoopsaProperty;
    use crate::protocol::struct_type::WoopsaStructType;
    use crate::protocol::value_type::WoopsaValueType;

    #[test]
    fn it_should_get_woopsaobject_root_path() {
        let root = WoopsaObject::root();
        assert_eq!(root.get_path(), "/");
    }

    #[test]
    fn it_should_get_woopsaobject_type_of() {
        let root = WoopsaObject::root();
        assert_eq!(root.type_of(), WoopsaStructType::WoopsaObject);
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

    #[test]
    fn it_should_get_woopsproperty_from_item() {
        let mut root = WoopsaObject::root();
        let mut objects = WoopsaObject::new(String::from("Objects"));

        let mut weatherstation = WoopsaObject::new(String::from("WeatherStation"));
        weatherstation.add_property(WoopsaProperty::new(
            String::from("Temperature"),
            String::from("21.5"),
            WoopsaValueType::Real,
        ));
        weatherstation.add_property(WoopsaProperty::new(
            String::from("IsRaining"),
            String::from("false"),
            WoopsaValueType::Logical,
        ));
        weatherstation.add_property(WoopsaProperty::new(
            String::from("Sensitivity"),
            String::from("0.5"),
            WoopsaValueType::Real,
        ));
        weatherstation.add_property(WoopsaProperty::new(
            String::from("Altitude"),
            String::from("430"),
            WoopsaValueType::Integer,
        ));
        weatherstation.add_property(WoopsaProperty::new(
            String::from("City"),
            String::from("Geneva"),
            WoopsaValueType::Text,
        ));

        let mut thermostat = WoopsaObject::new(String::from("Thermostat"));
        thermostat.add_property(WoopsaProperty::new(
            String::from("SetPoint"),
            String::from("24.3"),
            WoopsaValueType::Real,
        ));
        weatherstation.insert_item(thermostat);
        objects.insert_item(weatherstation);
        root.insert_item(objects);

        let thermostat = root.get_item_by_path(String::from("/Objects/WeatherStation/Thermostat"));
        let thermostat_set_point = thermostat.find_property_by_name(String::from("SetPoint"));
        assert_eq!(thermostat_set_point.get_value_type(), WoopsaValueType::Real);
        assert_eq!(thermostat_set_point.get_value(), String::from("24.3"));
    }
}
