local log = function(...)
  redis.log(redis.LOG_NOTICE, ...)
end

local HSET = function(key, field, val)
  return redis.call("HSET", key, field, val)
end

function xpendclaim(keys, args)
  log("xxx", #keys, keys[1])
  return 1
end
