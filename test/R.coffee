#!/usr/bin/env coffee

> ../index.js > Server conn

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

export default await conn(
  server, REDIS_USER, REDIS_PASSWORD, REDIS_DB
)
