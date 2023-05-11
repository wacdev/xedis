#!/usr/bin/env coffee

> @w5/redis_lua
  ./R
  @w5/read
  @w5/uridir
  @w5/dot
  path > join
  os > hostname
  msgpackr > unpack


await RedisLua(R).xpendclaim(
  read join uridir(import.meta),'xpendclaim.lua'
)

main = dot (stream)=> (redis, idle, limit, customer = hostname(), group='C') =>
  [
    => # xpendclaim
      r = await redis.fbin(
        'xpendclaim'
        [
          stream # stream
          group # group
          customer
        ]
        [
          idle    # idle
          limit   # limit
        ].map((i)=>''+i)
      )
      if r
        return unpack r
      []
  ]


[
  xpendclaim
] = main.task(
  R
  1e3 # 6e5 # idle
  30 #
)

console.log await xpendclaim()
