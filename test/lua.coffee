#!/usr/bin/env coffee

> @w5/redis_lua
  ./R
  @w5/read
  @w5/uridir
  path > join
  os > hostname
  msgpackr > unpack


await RedisLua(redis).xpendclaim(
  read uridir(import.meta),'xpendclaim.lua'
)

main = (redis, stream, idle, limit, customer = hostname(), group='C') = =>
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
        ]
      )
      if r.length
        return unpack r
      []
  ]


[
  xpendclaim
] = main(
  redis
  'task'
  6e5 # idle
)

console.log await xpendclaim()
