use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use crate::protocol::struct_type::WoopsaStructType;
use crate::protocol::value_type::WoopsaValueType;

use std::any::Any;
use std::fmt::Debug;

pub trait Value {
    fn type_of(&self) -> WoopsaStructType;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WoopsaValue {
    pub as_text: String,
    pub timestamp: i64,
    pub value_type: WoopsaValueType,
}

impl WoopsaValue {
    pub fn new(as_text: String, value_type: WoopsaValueType) -> WoopsaValue {
        let dt = Utc::now();
        WoopsaValue {
            as_text,
            timestamp: Utc::now().timestamp_nanos(),
            value_type,
        }
    }

    pub fn as_text(&self) -> Option<String> {
        if self.value_type == WoopsaValueType::Text {
            Some(self.as_text.clone())
        } else {
            None
        }
    }

    pub fn as_logical(&self) -> Option<bool> {
        if self.value_type == WoopsaValueType::Logical {
            Some(self.as_text.eq("true"))
        } else {
            None
        }
    }

    pub fn as_null(&self) -> Option<String> {
        if self.value_type == WoopsaValueType::Null {
            Some(String::from("Null"))
        } else {
            None
        }
    }

    pub fn as_datetime(&self) -> Option<NaiveDateTime> {
        if self.value_type == WoopsaValueType::DateTime {
            Some(NaiveDateTime::from_timestamp(
                self.as_text.parse::<i64>().unwrap(),
                0,
            ))
        } else {
            None
        }
    }

    pub fn timestamp(&self) -> i64 {
        self.timestamp
    }

    pub fn datetime_from_value_timestamp(&self) -> Option<NaiveDateTime> {
        if self.timestamp > 0 {
            Some(NaiveDateTime::from_timestamp(self.timestamp, 0))
        } else {
            None
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        if self.value_type == WoopsaValueType::Integer {
            Some(self.as_text.parse::<i64>().unwrap())
        } else {
            None
        }
    }

    pub fn as_i32(&self) -> Option<i32> {
        if self.value_type == WoopsaValueType::Integer {
            Some(self.as_text.parse::<i32>().unwrap())
        } else {
            None
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        if self.value_type == WoopsaValueType::Real {
            Some(self.as_text.parse::<f64>().unwrap())
        } else {
            None
        }
    }

    pub fn as_f32(&self) -> Option<f32> {
        if self.value_type == WoopsaValueType::Real {
            Some(self.as_text.parse::<f32>().unwrap())
        } else {
            None
        }
    }

    pub fn text(&self) -> String {
        self.as_text.clone()
    }

    pub fn as_str(&self) -> &str {
        self.as_text.as_str()
    }

    pub fn clone_with_timestamp_update(&self) -> WoopsaValue {
        WoopsaValue {
            as_text: self.as_text.clone(),
            timestamp: Utc::now().timestamp_nanos(),
            value_type: self.value_type.clone(),
        }
    }

    pub fn update_timestamp(&mut self) {
        self.timestamp = Utc::now().timestamp_nanos();
    }

    pub fn get_new_timestamp() -> i64 {
        Utc::now().timestamp_nanos()
    }

    pub fn eq(&self, value: WoopsaValue) -> bool {
        self.as_text == value.as_text
            && self.timestamp == value.timestamp
            && self.value_type == value.value_type
    }

    pub fn type_of(&self) -> WoopsaStructType {
        WoopsaStructType::WoopsaValue
    }
}

impl Value for WoopsaValue {
    fn type_of(&self) -> WoopsaStructType {
        self.type_of()
    }
}
