#!/usr/bin/env coffee

> ./R
  os > hostname

stream = 'task'
HOSTNAME = hostname()

rstr = =>
  Math.floor Math.random() * 100000

await R.xadd(
  stream
  [
    ['-',JSON.stringify({'<':rstr()})]
  ]
)

await R.xaddLi(
  stream
  [
    [
      ['-',JSON.stringify('A'+rstr())]
    ]
    [
      ['-',JSON.stringify('B'+rstr())]
    ]
  ]
)

