#!/usr/bin/env coffee

> ./index.js > Server conn
{
  REDIS_HOST_PORT
  REDIS_PASSWORD
} = process.env

[
  REDIS_HOST
  REDIS_PORT
  REDIS_DB
  REDIS_USER
] = REDIS_HOST_PORT.split(':')

REDIS_PORT = +REDIS_PORT or 6379

server = Server.hostPort REDIS_HOST, REDIS_PORT

I = await conn(
  server, REDIS_USER, REDIS_PASSWORD, REDIS_DB
)

key = new Uint8Array [2]

console.log await I.zrevrangebyscoreWithscore key
# map = '字典'
# key = 'xedis键'
# val = 'test测试'

# await I.hset(map, key,val)
#
# console.log '>>', await I.hget(map,key)
#
# await I.hset(map,key)
#
# console.log '>>', await I.hget(map,key)

# await I.hset(map, {'测试':key})

# console.log '>>', await I.hget(map,key)
# console.log '>>', await I.hget(map,'测试')
