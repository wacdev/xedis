#!/usr/bin/env coffee

> ./R
  os > hostname
  msgpackr > pack

stream = 'testTask'
HOSTNAME = hostname()

rstr = =>
  Math.floor Math.random() * 100000

await R.xadd(
  stream
  [
    [
      pack 1
      pack {'<':rstr()}
    ]
  ]
)

await R.xaddLi(
  stream
  [
    [
      [
        pack 2
        pack('A'+rstr())
      ]
    ]
    [
      [
        pack 3
        pack(['B'+rstr()])
      ]
    ]
  ]
)

