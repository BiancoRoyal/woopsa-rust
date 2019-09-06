pub mod constant;
pub mod container;
pub mod element;
pub mod error_type;
pub mod link;
pub mod method;
pub mod method_argument_info;
pub mod object;
pub mod property;
pub mod struct_type;
pub mod url;
pub mod value;
pub mod value_type;
pub mod verb;

#[cfg(test)]
mod integration_tests {
    use crate::protocol::object::WoopsaObject;
    use crate::protocol::property::WoopsaProperty;
    use crate::protocol::value_type::WoopsaValueType;

    #[test]
    fn it_build_simple_model() {
        let mut root = WoopsaObject::new(String::from("Root"));
        root.container
            .insert_item(WoopsaObject::new(String::from("Server")));
        let mut objects = WoopsaObject::new(String::from("Objects"));
        objects
            .container
            .insert_item(WoopsaObject::new(String::from("Devices")));
        objects
            .container
            .insert_item(WoopsaObject::new(String::from("Models")));

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

    #[test]
    fn it_build_weatherstation_model() {
        let mut root = WoopsaObject::root();
        let mut objects = WoopsaObject::new(String::from("Objects"));
        let mut weather_station = WoopsaObject::new(String::from("WeatherStation"));
        weather_station.add_property(WoopsaProperty::new(
            String::from("Temperature"),
            String::from("24.2"),
            WoopsaValueType::Real,
        ));
        weather_station.add_property(WoopsaProperty::new(
            String::from("IsRaining"),
            String::from("false"),
            WoopsaValueType::Logical,
        ));
        weather_station.add_property(WoopsaProperty::new(
            String::from("Sensitivity"),
            String::from("0.5"),
            WoopsaValueType::Real,
        ));
        weather_station.add_property(WoopsaProperty::new(
            String::from("Altitude"),
            String::from("430"),
            WoopsaValueType::Integer,
        ));
        weather_station.add_property(WoopsaProperty::new(
            String::from("City"),
            String::from("Geneva"),
            WoopsaValueType::Text,
        ));
        // Todo: weather_station.add_property(WoopsaProperty::new(String::from("Time"), String::from(SystemTime::now()), WoopsaValueType::DateTime));

        let mut thermostat = WoopsaObject::new(String::from("Thermostat"));
        thermostat.add_property(WoopsaProperty::new_readonly(
            String::from("SetPoint"),
            String::from("24.0"),
            WoopsaValueType::Real,
        ));
        weather_station.container.insert_item(thermostat);
        objects.container.insert_item(weather_station);
        root.container.insert_item(objects);

        println!("/Root/ is {}", &root);
        for (key, value) in &(root.container.items) {
            println!("item of root -> {}: {}", key, value);
        }

        let objects = root.find_item_by_name(String::from("Objects"));
        println!("/Root/Objects/ is {}", &objects);
        for (key, value) in &(objects.container.items) {
            println!("item of objects -> {}: {}", key, value);
        }

        let weather_station = objects.find_item_by_name(String::from("WeatherStation"));
        println!("/Root/Objects/WeatherStation/ is {}", &weather_station);
        for (key, value) in &(weather_station.properties) {
            println!("property of weather_station -> {}: {}", key, value);
        }
        for (key, value) in &(weather_station.container.items) {
            println!("item of weather_station -> {}: {}", key, value);
        }

        assert_eq!(objects.name(), String::from("Objects"));
    }
}
