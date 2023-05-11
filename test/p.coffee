#!/usr/bin/env coffee

> ./R
  os > hostname

stream = 'task'
HOSTNAME = hostname()

await R.xadd(
  stream
  [
    ['-',JSON.stringify(['good','yes'])]
  ]
)

await R.xaddLi(
  stream
  [
    [
      ['-',JSON.stringify(['1'])]
    ]
    [
      ['-',JSON.stringify(['2'])]
    ]
  ]
)

