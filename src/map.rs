use std::collections::HashMap;

use napi::{
  bindgen_prelude::{FromNapiValue, Object, TypeName, ValidateNapiValue},
  sys::{napi_env, napi_value},
  Result, ValueType,
};

use crate::bin::Bin;

pub struct Map(pub HashMap<String, Box<[u8]>>);

impl FromNapiValue for Map {
  unsafe fn from_napi_value(env: napi_env, napi_val: napi_value) -> Result<Self> {
    let mut r = HashMap::new();
    let obj = Object::from_napi_value(env, napi_val)?;
    for i in Object::keys(&obj)?.into_iter() {
      let val: Bin = obj.get(&i)?.unwrap();
      r.insert(i, val.into());
    }
    Ok(Self(r))
  }
}

impl ValidateNapiValue for Map {}

impl TypeName for Map {
  fn type_name() -> &'static str {
    "Map"
  }

  fn value_type() -> ValueType {
    ValueType::Object
  }
}
