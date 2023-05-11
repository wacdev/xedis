#!/usr/bin/env coffee

> @w5/redis_lua
  @w5/redis_lua/dot_bind
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

B = DotBind R
B.fbin.xpendclaim

main = dot (stream)=>
  (
    redis
    idle
    limit
    group
    customer = hostname()
  ) =>
    [
      => # xpendclaim
        r = await redis.xpendclaim(
          stream # stream
          group # group
          customer
        )(
          idle    # idle
          limit   # limit
        )
        if r
          r = unpack r
          # for [id, retry, ...kv] from r
          #   console.log id, retry, kv
          return r
        []
    ]


stream = 'testTask'
group = 'C'
[
  xpendclaim
] = main[stream](
  R
  1e3 # 6e5 # idle
  3 #
  group
)

for [id,retry,...args] from await xpendclaim()
  console.log id,retry,args
  await R.xack stream, group, id


