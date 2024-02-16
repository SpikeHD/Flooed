use std::collections::HashMap;

use miniserde::json::{Object, Value};

// Create a to_object function for values
pub trait ToObject {
  fn to_object(&self) -> Object;
}

pub trait ToString {
  fn to_string(&self) -> String;
}

// ToValue, allows things like hashmaps to be converted to Value
pub trait ToValue {
  fn to_value(&self) -> Value;
}

impl ToObject for Value {
  fn to_object(&self) -> Object {
    match self {
      Value::Object(obj) => obj.clone(),
      _ => Object::new(),
    }
  }
}

impl ToString for Value {
  fn to_string(&self) -> String {
    match self {
      Value::String(s) => s.clone(),
      _ => String::new(),
    }
  }
}

