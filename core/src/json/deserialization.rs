extern crate serde_json;

trait Deserialization {

}

struct WoopsaDeserialization {
    input: serde_json::Value,
    output: String
}

impl Deserialization for WoopsaDeserialization {

}