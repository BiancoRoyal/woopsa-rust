extern crate serde_json;

trait Serialization {

}

struct WoopsaSerialization {
    input: String,
    output: serde_json::Value
}

impl Serialization for WoopsaSerialization {

}
