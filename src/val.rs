use std::fmt::{Debug, Formatter};

use fred::{
  prelude::{FromRedis, RedisError},
  types::RedisValue,
};
use napi::{
  bindgen_prelude::{ToNapiValue, Uint8Array, Undefined},
  sys::{napi_env, napi_value},
};

pub struct Val(Option<Uint8Array>);

impl Debug for Val {
  fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
    fmt.write_str("Val:")?;
    let msg = match &self.0 {
      Some(val) => std::string::String::from_utf8_lossy(val),
      None => "None".into(),
    };
    fmt.write_str(&msg)
  }
}

impl ToNapiValue for Val {
  unsafe fn to_napi_value(env: napi_env, this: Self) -> Result<napi_value, napi::Error> {
    match this.0 {
      Some(buf) => Uint8Array::to_napi_value(env, buf),
      None => Undefined::to_napi_value(env, ()),
    }
  }
}
impl FromRedis for Val {
  fn from_value(val: RedisValue) -> Result<Self, RedisError> {
    Ok(match val.as_bytes() {
      None => Val(None),
      Some(v) => Val(Some(v.into())),
    })
  }
}
