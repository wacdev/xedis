use anyhow::Result;
use fred::{
  interfaces::{
    ClientLike, FunctionInterface, HashesInterface, KeysInterface, SetsInterface,
    SortedSetsInterface,
  },
  prelude::{Expiration, ReconnectPolicy, RedisClient, RedisConfig, ServerConfig},
  types::{RedisMap, SetOptions, ZRange, ZRangeBound, ZRangeKind},
};
use napi::bindgen_prelude::{Either, Either3};
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
            $name:ident $($arg:ident:$arg_ty:ty)* : $func:ident
        )*
    ) => {
        #[napi]
        impl Xedis {
            $(
                #[napi]
                pub async fn $name(&self, $($arg:$arg_ty,)* key: Either<Vec<Bin>,Bin>) -> Result<u32> {
                    Ok(
                        match key{
                            Either::A(key)=>self.c.$func($($arg,)* key),
                            Either::B(key)=>self.c.$func($($arg,)* key)
                        }.await?
                    )
                }
            )*
        }
    };
}

def_one_or_li!(
    del : del
    exist : exists
    zrem key:Bin: zrem
);

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
fcall name:Bin key:Vec<Bin> val:Vec<Bin> => () : fcall
fcall_r name:Bin key:Vec<Bin> val:Vec<Bin> => () : fcall_ro
}

macro_rules! fcall {
    ($($name:ident $name_r:ident $rt:ty;)*)=>{
        def!{
            $(
                $name name:Bin key:Vec<Bin> val:Vec<Bin> => Option<$rt> : fcall
                $name_r name:Bin key:Vec<Bin> val:Vec<Bin> => Option<$rt> : fcall_ro
            )*
        }
    };
    ($($name:ident $rt:ty;)*)=>{
        paste!{
            fcall!(
                $(
                    $name [<$name _r>] $rt;
                )*
            );
        }
    }
}

fcall!(
  fbool bool;
  fbin Val;
  fnum f64;
  fstr String;
);

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

def_with_args!(
    setex key:Bin val:Bin ex:i64 => () {
        set(key, val, Some(Expiration::EX(ex)), None, false)
    }

    fnload code:Bin => String {
        function_load(true, code)
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

macro_rules! zadd {
  ($($name:ident $set_opt:expr)*) => {
    #[napi]
    impl Xedis {
      $(
      #[napi]
      pub async fn $name(
        &self,
        zset: Bin,
        key: Either3<HashMap<String, f64>, Vec<(Bin, f64)>, Bin>,
        score: Option<f64>,
      ) -> Result<u32> {
        Ok(
          if let Some(score) = score {
            // https://docs.rs/fred/6.2.1/fred/interfaces/trait.SortedSetsInterface.html#method.zadd
            match key {
              Either3::C(key) => self.c.zadd(zset, $set_opt, None, false, false, (score, key)),
              _ => unreachable!(),
            }
          } else {
            match key {
              Either3::A(key) => self.c.zadd(
                zset,
                $set_opt,
                None,
                false,
                false,
                key.into_iter().map(|(k, s)| (s, k)).collect::<Vec<_>>(),
              ),
              Either3::B(key) => self.c.zadd(
                zset,
                $set_opt,
                None,
                false,
                false,
                key.into_iter().map(|(k, s)| (s, k)).collect::<Vec<_>>(),
              ),
              Either3::C(key) => self.c.zrem(zset, key),
            }
          }
          .await?,
        )
      }
      )*
    }
  };
}

zadd!(

zadd None

// 已存在时才设置
zadd_xx Some(SetOptions::XX)

// 不存在时才设置
zadd_nx Some(SetOptions::NX)

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

zset_range!(zrangebyscore_withscore (Val, f64) : zrangebyscore min max true);
zset_range!(zrangebyscore Val : zrangebyscore  min max false);

zset_range!(zrevrangebyscore_withscore (Val, f64) : zrevrangebyscore max min true);
zset_range!(zrevrangebyscore Val : zrevrangebyscore  max min false);
