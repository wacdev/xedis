#!/usr/bin/env coffee

> @w5/redis_lua
  ./R
  @w5/read
  @w5/uridir
  path > join
  chalk
  os > hostname

customer = hostname()

{greenBright} = chalk

readLua = (name)=>
  read join uridir(import.meta),name+'.lua'

lua = readLua('test')

await RedisLua(R).XedisTest(lua)

console.log(
  JSON.parse await R.fstr(
    'xpendclaim'
    [
      'task' # stream
      'R' # group
      customer # customer
    ]
    [
      '5000' # idle
      '2'   # limit
    ]
  )
)
