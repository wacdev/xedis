use fred::{
  bytes_utils::Str,
  prelude::RedisError,
  types::{MultipleKeys, MultipleStrings, MultipleValues, RedisKey, RedisValue},
};
use napi::{
  bindgen_prelude::{
    Buffer, Either, Either4, FromNapiValue, ToNapiValue, TypeName, Uint8Array, ValidateNapiValue,
  },
  sys::{napi_env, napi_value},
  Result, ValueType,
};

pub type StringUint8Array = Either4<f64, String, Buffer, Uint8Array>;
pub struct Bin(pub StringUint8Array);

impl From<Bin> for Box<[u8]> {
  fn from(val: Bin) -> Self {
    match &val.0 {
      Either4::A(x) => {
        if x.fract() == 0.0 {
          (*x as i64).to_string().as_bytes().into()
        } else {
          x.to_string().as_bytes().into()
        }
      }
      Either4::B(x) => x.as_bytes().into(),
      Either4::C(x) => x.as_ref().into(),
      Either4::D(x) => x.as_ref().into(),
    }
  }
}

impl From<Bin> for Str {
  fn from(val: Bin) -> Self {
    std::string::String::from_utf8_lossy(&Into::<Box<[u8]>>::into(val)).into()
  }
}

// Into<StrInner<fred::bytes::Bytes>> impl StrInner<fred::bytes::Bytes> for Bin {}

impl ToNapiValue for Bin {
  unsafe fn to_napi_value(env: napi_env, bin: Self) -> napi::Result<napi_value> {
    let bin = &Into::<Box<[u8]>>::into(bin)[..];
    Buffer::to_napi_value(env, bin.into())
  }
}

impl FromNapiValue for Bin {
  unsafe fn from_napi_value(env: napi_env, napi_val: napi_value) -> napi::Result<Self> {
    Ok(Bin(StringUint8Array::from_napi_value(env, napi_val)?))
  }
}

impl From<Bin> for RedisValue {
  fn from(t: Bin) -> RedisValue {
    RedisValue::from(Into::<Box<[u8]>>::into(t))
  }
}

impl From<Bin> for RedisKey {
  fn from(t: Bin) -> RedisKey {
    RedisKey::from(Into::<Box<[u8]>>::into(t))
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

pub type EitherVecBinOrBin = Either<Vec<Bin>, Bin>;
pub struct VecBinOrBin(pub EitherVecBinOrBin);

impl FromNapiValue for VecBinOrBin {
  unsafe fn from_napi_value(env: napi_env, napi_val: napi_value) -> napi::Result<Self> {
    Ok(VecBinOrBin(EitherVecBinOrBin::from_napi_value(
      env, napi_val,
    )?))
  }
}

impl TryFrom<VecBinOrBin> for MultipleValues {
  type Error = RedisError;
  fn try_from(t: VecBinOrBin) -> std::result::Result<MultipleValues, RedisError> {
    Ok(match t.0 {
      Either::A(t) => t.try_into()?,
      Either::B(t) => t.into(),
    })
  }
}

impl From<VecBinOrBin> for MultipleStrings {
  fn from(t: VecBinOrBin) -> MultipleKeys {
    match t.0 {
      Either::A(t) => t.into(),
      Either::B(t) => t.into(),
    }
  }
}
