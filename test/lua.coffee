#!/usr/bin/env coffee

> @w5/redis_lua
  ./R
  @w5/read
  @w5/uridir
  path > join
  os > hostname

customer = hostname()

xpendclaim = =>
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

lua = (func, R, name, fp)=>
  (args...)=>
    try
      return await func(...args)
    catch err
      if err.code == 'GenericFailure' and err.message == 'Unknown Error: ERR Function not found'
        await RedisLua(R)[name](read fp)
        return await func(...args)
      else
        throw err
    return

xpendclaim = lua(
  xpendclaim,
  R,
  'XedisTest',
  join uridir(import.meta),'test.lua'
)

console.log await xpendclaim()
