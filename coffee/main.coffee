#!/usr/bin/env coffee

> ava
  @w5/utf8/utf8e.js
  @w5/utf8/utf8d.js
  ../index.js > Server conn
  assert > strict:assert
  os

{
  REDIS_HOST_PORT
  REDIS_PASSWORD
  REDIS_USER
  REDIS_DB
} = process.env

[
  REDIS_HOST
  REDIS_PORT
] = REDIS_HOST_PORT.split(':')


REDIS_PORT = +REDIS_PORT or 6379

KEY = 'xedisTest.'+os.arch()+'.'+os.type()+'.node-'+process.version+'.'
{glibcVersionRuntime} = process.report.getReport().header
if glibcVersionRuntime
  KEY+=(glibcVersionRuntime+'.')

+ C

ava.before =>
  server = Server.hostPort REDIS_HOST, REDIS_PORT

  C = await conn(
    server, REDIS_USER, REDIS_PASSWORD, REDIS_DB
  )
  return


ava(
  'zset'
  (t)=>
    zset = KEY+'zset'
    key = '测试键'
    score = 1.23
    await C.del zset
    t.is score, await C.zincrby zset, key, score
    t.is score*2, await C.zincrby zset, key, score
    t.is score, await C.zincrby zset, key, -score
    t.is score, await C.zscore zset, key
    t.is score+1, await C.zincr zset, key
    t.is score+1, await C.zscore zset, key
    # t.deepEqual [utf8e(key)], await C.zrangebyscore(zset)
    key2 = key+2
    keye = utf8e key
    key2e = utf8e(key2)
    await C.zincrby zset, key2, score
    t.deepEqual [key2e,keye], await C.zrangebyscore(zset)
    t.deepEqual(
      await C.zrangebyscore(
        zset
        offset:1
      )
      [keye]
    )
    t.deepEqual(
      await C.zrevrangebyscore(
        zset
        offset:1
      )
      [key2e]
    )
    t.deepEqual [key2e], await C.zrangebyscore(
      zset
      limit:1
    )
    t.deepEqual [key2e], await C.zrangebyscore(
      zset, max:score
    )

    t.deepEqual [keye], await C.zrangebyscore(
      zset, min:score+1
    )
    t.deepEqual(
      (
        await C.zrangebyscore(
          zset
          min:'('+score
        )
      ).map utf8d
      [key]
    )
    t.deepEqual(
      (
        await C.zrangebyscore(
          zset
          max:'('+score
        )
      ).map utf8d
      []
    )
    t.deepEqual(
      await C.zrevrangebyscoreWithscore(zset)
      [
        [
          keye
          score+1
        ]
        [
          key2e
          score
        ]
      ]
    )

    t.deepEqual(
      await C.zrangebyscoreWithscore(zset)
      [
        [
          key2e
          score
        ]
        [
          keye
          score+1
        ]
      ]
    )

    t.is 2, await C.zrem(zset,[key,key2])

    t.is(
      await C.zadd zset, key, score
      1
    )
    t.is(
      await C.zadd zset, [
        [key, score]
        [key2, score]
      ]
      1
    )
    t.is(
      await C.zadd zset, {a:1.2,b:3.2,c:1.2}
      3
    )
    t.is(
      await C.zaddNx zset,'a',1
      0
    )
    t.is(
      await C.zaddXx zset,'t',1
      0
    )
    await C.del zset
    t.is null, await C.zscore zset, key
    return
)

ava(
  'set'
  (t)=>
    set = KEY+'set'
    await C.del set
    val = 'val'
    t.is 1, await C.sadd set, val
    t.deepEqual [utf8e(val)], await C.smembers set

    val2 = '2'
    val3 = 3
    await C.sadd set, [val2, val3]

    t.deepEqual(
      await C.smismember(set, ['not exist', val, val2, 'not exist 2', 3])
      [false, true, true, false, true]
    )
    await C.del set
    return
)

ava(
  'hset'
  (t)=>
    map = KEY+'hset'
    key = '键'
    val = '值'
    await C.del(map)
    await C.hset(map, key, val)
    t.is val,await C.hget(map, key)
    t.deepEqual utf8e(val),await C.hgetB(map, key)
    await C.hdel map, key
    t.is null, await C.hget(map, key)
    await C.del(map)
    t.is 1, await C.hincrby map, key, 1
    t.is 6, await C.hincrby map, key, 5
    t.is -4, await C.hincrby map, key, -10
    t.is -3, await C.hincr map, key

    await C.del(map)
    await C.hmset(map, [[1,2],[3,4]])
    t.is (await C.hget(map,1)),'2'
    t.is (await C.hget(map,3)),'4'
    await C.del(map)
    t.is null,await C.hget(map, key)

    return
)

ava(
  'get'
  (t)=>
    key = KEY+'get'
    key2 = key+2
    val = 'test测试'
    await C.del(key)
    await C.del(key2)
    await C.set(key,val)
    await C.set(key2,val)

    t.is await C.exist(key), 1
    t.is await C.exist([key,key2]), 2
    t.is await C.exist([key,key,key2]), 3

    t.deepEqual utf8e(val), await C.getB(key)
    t.deepEqual val, await C.get(key)
    t.is await C.del([key2,key]), 2
    t.is 0, await C.del(key)
    t.is null,await C.get(key)
    t.is null,await C.get(key2)
    return
)

