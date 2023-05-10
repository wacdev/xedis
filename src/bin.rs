use fred::{
  bytes_utils::Str,
  types::{RedisKey, RedisValue},
};
use napi::{
  bindgen_prelude::{Buffer, FromNapiValue, ToNapiValue, TypeName, ValidateNapiValue},
  sys::{napi_env, napi_value},
  Either, Result, ValueType,
};
pub type StringUint8Array = Either<String, Buffer>;
pub struct Bin(pub StringUint8Array);

impl AsRef<[u8]> for Bin {
  fn as_ref(&self) -> &[u8] {
    self.0.as_ref()
  }
}

impl Into<Str> for Bin {
  fn into(self) -> Str {
    std::string::String::from_utf8_lossy(self.as_ref()).into()
  }
}

// Into<StrInner<fred::bytes::Bytes>> impl StrInner<fred::bytes::Bytes> for Bin {}

impl ToNapiValue for Bin {
  unsafe fn to_napi_value(env: napi_env, bin: Self) -> napi::Result<napi_value> {
    Buffer::to_napi_value(env, bin.as_ref().into())
  }
}

impl FromNapiValue for Bin {
  unsafe fn from_napi_value(env: napi_env, napi_val: napi_value) -> napi::Result<Self> {
    Ok(Bin(StringUint8Array::from_napi_value(env, napi_val)?))
  }
}

impl From<Bin> for RedisValue {
  fn from(t: Bin) -> RedisValue {
    RedisValue::from(t.as_ref())
  }
}

impl From<Bin> for RedisKey {
  fn from(t: Bin) -> RedisKey {
    RedisKey::from(t.as_ref())
  }
}

impl ValidateNapiValue for Bin {
  unsafe fn validate(env: napi_env, napi_val: napi_value) -> Result<napi_value> {
    match Buffer::validate(env, napi_val) {
      Ok(r) => Ok(r),
      Err(_err) => String::validate(env, napi_val),
    }
  }
}

impl TypeName for Bin {
  fn type_name() -> &'static str {
    "Bin"
  }

  fn value_type() -> ValueType {
    ValueType::Object
  }
}
