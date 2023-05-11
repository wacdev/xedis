#!/usr/bin/env coffee

> ./R
  os > hostname
  @w5/utf8/utf8d
  msgpackr > unpack

stream = 'testTask'
group = 'C'
customer = hostname()

n = 0
while n++ < 100
  for [task,li] from await R.xnext(
    group
    customer
    10 # limit
    6e5 # block
    false # noack
    stream
  )
    console.log utf8d task
    for [id, msg] from li
      console.log id
      for [k,v] from msg
        console.log unpack(k),unpack(v)

# key = new Uint8Array [2]
#
# console.log await I.zrevrangebyscoreWithscore key
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
