use anyhow::Result;
use fred::{
  interfaces::{ClientLike, HashesInterface, KeysInterface, SetsInterface, SortedSetsInterface},
  prelude::{Expiration, ReconnectPolicy, RedisClient, RedisConfig, ServerConfig},
  types::{RedisMap, ZRange, ZRangeBound, ZRangeKind},
};
use napi::Either;
use paste::paste;
pub type OptionString = Option<String>;
use std::collections::HashMap;

use napi_derive::napi;

pub mod bin;
pub mod map;
pub mod val;

use bin::Bin;
use map::Map;
use val::Val;

const MIN: ZRange = ZRange {
  kind: ZRangeKind::Inclusive,
  range: ZRangeBound::NegInfiniteScore,
};

const MAX: ZRange = ZRange {
  kind: ZRangeKind::Inclusive,
  range: ZRangeBound::InfiniteScore,
};

pub type StrOrN = Either<String, f64>;
pub type BinOrMap = Either<Bin, Map>;

#[napi]
pub struct Xedis {
  c: RedisClient,
}
macro_rules! i64 {
  ($opt:ident,$key:ident,$default:expr) => {
    match $opt.get(stringify!($key)) {
      None => $default,
      Some(t) => match t {
        Either::A(t) => t.parse()?,
        Either::B(t) => (*t as i64),
      },
    }
  };
}

macro_rules! zrange {
  ($opt:ident,$kind:ident,$default:ident) => {
    match $opt.get(stringify!($kind)) {
      Some(v) => match v {
        Either::A(v) => v.into(),
        Either::B(v) => ZRange {
          kind: ZRangeKind::Inclusive,
          range: ZRangeBound::Score(*v),
        },
      },
      None => $default,
    }
  };
}

macro_rules! opt_mlo {
  ($opt:ident) => {{
    let min;
    let max;
    let limit_offset;
    if let Some(opt) = $opt {
      min = zrange!(opt, min, MIN);
      max = zrange!(opt, max, MAX);
      let limit = i64!(opt, limit, -1);
      let offset = i64!(opt, offset, 0);
      limit_offset = if limit == -1 && offset == 0 {
        None
      } else {
        Some((offset, limit))
      }
    } else {
      limit_offset = None;
      min = MIN;
      max = MAX;
    }
    (min, max, limit_offset)
  }};
}

#[napi]
pub struct Server {
  c: ServerConfig,
}

#[napi]
impl Server {
  #[napi(factory)]
  pub fn cluster(host_port_li: Vec<(String, u16)>) -> Self {
    Self {
      c: ServerConfig::Clustered {
        hosts: host_port_li
          .into_iter()
          .map(|(host, port)| fred::types::Server {
            host: host.into(),
            port,
            tls_server_name: None,
          })
          .collect(),
      },
    }
  }

  #[napi(factory)]
  pub fn host_port(host: String, port: u16) -> Self {
    Self {
      c: ServerConfig::Centralized {
        server: fred::types::Server {
          host: host.into(),
          port,
          tls_server_name: None,
        },
      },
    }
  }
}

#[napi]
pub async fn conn(
  server: &Server,
  username: OptionString,
  password: OptionString,
  database: Option<u8>,
) -> Result<Xedis> {
  let mut conf = RedisConfig {
    version: fred::types::RespVersion::RESP3,
    ..Default::default()
  };
  conf.server = server.c.clone();
  conf.username = username;
  conf.password = password;
  conf.database = database;
  /*
  https://docs.rs/fred/6.2.1/fred/types/enum.ReconnectPolicy.html#method.new_constant
  */
  let policy = ReconnectPolicy::new_constant(6, 1);
  let client = RedisClient::new(conf, None, Some(policy));
  client.connect();
  client.wait_for_connect().await?;
  Ok(Xedis { c: client })
}

macro_rules! def_one_or_li {
    (
        $(
            $name:ident : $func:ident
        )*
    ) => {
        #[napi]
        impl Xedis {
            $(
                #[napi]
                pub async fn $name(&self, key: Either<Vec<Bin>,Bin>) -> Result<i64> {
                    Ok(
                        match key{
                          Either::A(key)=>self.c.$func(key),
                          Either::B(key)=>self.c.$func(key)
                        }.await?
                    )
                }
            )*
        }
    };
}

macro_rules! def {
    (
        $(
            $name:ident
            $($arg:ident:$arg_ty:ty)*
            =>
            $rt:ty : $func:ident
        )*
    ) => {
        #[napi]
        impl Xedis {
            $(
                #[napi]
                pub async fn $name(&self, $($arg:$arg_ty),*) -> Result<$rt> {
                    Ok(self.c.$func($($arg),*).await?)
                }
            )*
        }
    };
}

macro_rules! def_with_args {
    (
        $(
            $name:ident
            $($arg:ident:$arg_ty:ty)*
            =>
            $rt:ty {
                $($more:tt)*
            }
        )*
    ) => {
        #[napi]
        impl Xedis {
            $(
                #[napi]
                pub async fn $name(&self, $($arg:$arg_ty),*) -> Result<$rt> {
                    Ok(self.c.$($more)*.await?)
                }
            )*
        }
    };
}
// macro_rules! fcall_ro {
//   ($cx:ident, $ty:ty)=>{{
//     let name = to_str($cx, 1)?;
//     let keys = to_bin_li($cx, 2)?;
//     let vals = to_bin_li($cx, 3)?;
//     this!($cx this {
//       this.fcall_ro::<$ty,_,_,_>(
//         name,
//         keys,
//         vals,
//       )
//     })
//   }}
// }
//
// macro_rules! fcall{
//   ($cx:ident, $ty:ty)=>{{
//     let name = to_str($cx, 1)?;
//     let keys = to_bin_li($cx, 2)?;
//     let vals = to_bin_li($cx, 3)?;
//     if keys.len() > 0{
//       this!($cx this {
//         this.fcall::<$ty,_,_,_>(
//           name,
//           keys,
//           vals,
//         )
//       })
//     } else {
//       this!($cx this {
//         this.fcall_ro::<$ty,_,_,_>(
//           name,
//           keys,
//           vals,
//         )
//       })
//     }
//   }}
// }
//
// #[macro_export]
// macro_rules! def_fn {
//   ($fn:ident |$cx:ident| $body:tt) => {
//   nlib::paste! {
//     pub fn $fn(mut $cx: Cx) -> JsResult<JsValue> {
//     let $cx = &mut $cx;
//     $body
//     }
//   }
//   };
//
//   ($($fn:ident |$cx:ident| $body:block)+) => {
//   $(
//     def_fn!($fn |$cx| $body);
//   )+
//   }
// }
//

def! {
expire key:Bin ex:i64 => bool : expire
get key:Bin => OptionString : get
get_b key:Bin => Val : get
hdel map:Bin key:Bin => u32 : hdel
hexist map:Bin key:Bin => bool : hexists
hget map:Bin key:Bin => OptionString : hget
hget_b map:Bin key:Bin => Val : hget
hincrby map:Bin key:Bin val:i64 => i64 : hincrby
hmget map:Bin li:Vec<Bin> => Vec<OptionString> : hmget
hmget_b map:Bin li:Vec<Bin> => Vec<Val> : hmget
quit => () : quit
sadd set:Bin val:Bin => i64 : sadd
smembers set:Bin => Vec<Val> : smembers
zscore zset:Bin key:Bin => Option<f64> : zscore
}

def_with_args!(
setex key:Bin val:Bin ex:i64 => () {
    set(key, val, Some(Expiration::EX(ex)), None, false)
}

hincr map:Bin key:Bin => i64 {
    hincrby(map, key, 1)
}

zincrby zset:Bin key:Bin score: f64 => f64 {
    zincrby(zset,score,key)
}

zincr zset:Bin key:Bin=> f64 {
    zincrby(zset,1.0,key)
}

set key:Bin val:Bin => () {
    // https://docs.rs/fred/6.2.1/fred/interfaces/trait.KeysInterface.html#method.set
    set(key,val,None,None,false)
}


);

def_one_or_li!(
    del : del
    exist : exists
);

#[napi]
impl Xedis {
  #[napi]
  pub async fn hset(&self, map: Bin, key: BinOrMap, val: Option<Bin>) -> Result<()> {
    let map = map.as_ref();
    Ok(
      self
        .c
        .hset::<(), _, _>(
          map,
          match key {
            napi::Either::A(key) => match val {
              Some(val) => TryInto::<RedisMap>::try_into(vec![(key, val)])?,
              None => {
                self.c.hdel(map, key).await?;
                return Ok(());
              }
            },
            napi::Either::B(key) => key.0.try_into()?,
          },
        )
        .await?,
    )
  }
}

macro_rules! zset_range {
  ($name:ident $rt:ty : $func:ident $m1:ident $m2:ident $score:ident) => {
    #[napi]
    impl Xedis {
      #[napi]
      pub async fn $name(
        &self,
        zset: Bin,
        opt: Option<HashMap<String, StrOrN>>,
      ) -> Result<Vec<$rt>> {
        let (min, max, limit_offset) = opt_mlo!(opt);
        paste! {
            Ok(self.c.$func(zset, [<$m1>], [<$m2>], $score, limit_offset).await?)
        }
      }
    }
  };
}

zset_range!(zrangebyscore_withscores (Val, f64) : zrangebyscore min max true);
zset_range!(zrangebyscore Val : zrangebyscore  min max false);

zset_range!(zrevrangebyscore_withscores (Val, f64) : zrevrangebyscore max min true);
zset_range!(zrevrangebyscore Val : zrevrangebyscore  max min false);

//   redis_zrem |cx| {
//     let a1 = to_bin(cx, 1)?;
//     let a2 = args_bin_li(cx, 2)?;
//
//     this!(cx this {
//       this.zrem::<f64,_,_>(
//         a1,
//         a2
//       )
//     })
//   }
//
//   redis_zadd |cx| {
//     let a1 = to_bin(cx, 1)?;
//     let a2 = to_bin(cx, 2)?;
//     let a3 = as_f64(cx, 3)?;
//     this!(cx this {
//       this.zadd::<f64,_,_>(
//         a1,
//         None,
//         None,
//         false,
//         false,
//         (
//           a3,
//           a2,
//         )
//       )
//     })
//   }
//
//   redis_zadd_xx |cx| {
//     let a1 = to_bin(cx, 1)?;
//     let a2 = to_bin(cx, 2)?;
//     let a3 = as_f64(cx, 3)?;
//
//     this!(cx this {
//       this.zadd::<f64,_,_>(
//         a1,
//         Some(SetOptions::XX),
//         None,
//         false,
//         false,
//         (
//           a3,
//           a2,
//         )
//       )
//     })
//   }
//
//
//   redis_fcall |cx| { fcall!(cx,()) }
//   redis_fcall_r |cx| { fcall_ro!(cx,()) }
//   redis_fbool |cx| { fcall!(cx,Option<bool>) }
//   redis_fbool_r |cx| { fcall_ro!(cx,Option<bool>) }
//   redis_fbin |cx| { fcall!(cx,Option<Vec<u8>>) }
//   redis_fbin_r |cx| { fcall_ro!(cx,Option<Vec<u8>>) }
//   redis_fnum |cx| { fcall!(cx,Option<f64>) }
//   redis_fnum_r |cx| { fcall_ro!(cx,Option<f64>) }
//   redis_fstr |cx| { fcall!(cx,OptionString) }
//   redis_fstr_r |cx| { fcall_ro!(cx,OptionString) }
//
//   redis_get |cx| {
//   let a1 = to_bin(cx, 1)?;
//   this!(cx this { this.get::<OptionString, _>(a1) })
//   }
//
//   redis_fnload |cx| {
//   let a1 = to_str(cx, 1)?;
//   this!(cx this { this.function_load::<String, _>(true, a1) })
//   }
//
// }
